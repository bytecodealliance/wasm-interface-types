use crate::ast::{self, kw};
use wast::parser::{Parse, Parser, Result};

/// A `*.wit` file in its entirety.
///
/// This is typically what you're parsing at the top-level.
pub struct Wit<'a> {
    /// The module that this `*.wit` file contained.
    pub module: Module<'a>,
}

impl<'a> Parse<'a> for Wit<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let module = parser.parens(|parser| parser.parse())?;
        Ok(Wit { module })
    }
}

/// A WebAssembly interface types-enhanced module.
pub struct Module<'a> {
    /// The core WebAssembly module which doesn't use interface types.
    pub core: wast::Module<'a>,
    /// The various `@interface` adapter directives representing the wasm
    /// interface types of this module.
    pub adapters: Vec<Adapter<'a>>,
}

impl Module<'_> {
    /// Encodes this `Module` into its binary form.
    pub fn encode(&mut self) -> std::result::Result<Vec<u8>, wast::Error> {
        let names = self.core.resolve()?;
        let core = match &self.core.kind {
            wast::ModuleKind::Text(list) => &list[..],
            wast::ModuleKind::Binary(_) => &[],
        };
        crate::resolve::resolve(core, &mut self.adapters, &names)?;
        let mut core = self.core.encode()?;
        core.extend_from_slice(&crate::binary::encode(&self.adapters));
        Ok(core)
    }
}

impl<'a> Parse<'a> for Module<'a> {
    fn parse(parser: Parser<'a>) -> Result<Module<'a>> {
        let span = parser.parse::<kw::module>()?.0;
        let name = parser.parse()?;
        let mut fields = Vec::new();
        let mut adapters = Vec::new();
        while !parser.is_empty() {
            parser.parens(|parser| {
                if parser.peek::<ast::AtInterface>() {
                    adapters.push(parser.parse()?);
                } else {
                    fields.push(parser.parse()?);
                }
                Ok(())
            })?;
        }
        Ok(Module {
            core: wast::Module {
                span,
                name,
                kind: wast::ModuleKind::Text(fields),
            },
            adapters,
        })
    }
}

/// List of possible `@interface` adapters that can be listed in a module.
pub enum Adapter<'a> {
    /// An interface type definition (function signature).
    Type(ast::Type<'a>),
    /// An import definition using interface types as a function signature.
    Import(ast::Import<'a>),
    /// An export using wasm interface types as a function signature.
    Export(ast::Export<'a>),
    /// An adapter function using wasm interface types.
    Func(ast::Func<'a>),
    /// A connection between a core wasm import and an inline defined function.
    Implement(ast::Implement<'a>),
}

impl<'a> Parse<'a> for Adapter<'a> {
    fn parse(parser: Parser<'a>) -> Result<Adapter<'a>> {
        parser.parse::<ast::AtInterface>()?;
        let mut l = parser.lookahead1();
        if l.peek::<kw::r#type>() {
            return Ok(Adapter::Type(parser.parse()?));
        }
        if l.peek::<kw::import>() {
            return Ok(Adapter::Import(parser.parse()?));
        }
        if l.peek::<kw::export>() {
            return Ok(Adapter::Export(parser.parse()?));
        }
        if l.peek::<kw::func>() {
            return Ok(Adapter::Func(parser.parse()?));
        }
        if l.peek::<kw::implement>() {
            return Ok(Adapter::Implement(parser.parse()?));
        }
        Err(l.error())
    }
}

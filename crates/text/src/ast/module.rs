use crate::ast::{self, annotation, kw};
use wast::parser::{Parse, Parser, Result};

/// A `*.wat` file in its entirety, including wasm interface types.
///
/// This is typically what you're parsing at the top-level.
pub struct Wat<'a> {
    /// The module that this `*.wat` file contained.
    pub module: Module<'a>,
}

impl<'a> Parse<'a> for Wat<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let module = if !parser.peek2::<kw::module>() {
            parse_text_module(parser.cur_span(), None, None, parser)?
        } else {
            parser.parens(|parser| parser.parse())?
        };
        Ok(Wat { module })
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
        crate::binary::append(&self.adapters, &mut core);
        Ok(core)
    }
}

impl<'a> Parse<'a> for Module<'a> {
    fn parse(parser: Parser<'a>) -> Result<Module<'a>> {
        // Parse `(module binary ...)` here, noting that we don't parse the
        // binary itself when looking for wasm adapters.
        if parser.peek2::<kw::binary>() {
            return Ok(Module {
                core: parser.parse()?,
                adapters: Vec::new(),
            });
        }

        let span = parser.parse::<kw::module>()?.0;
        let name = parser.parse()?;
        let annotation = parser.parse()?;
        parse_text_module(span, name, annotation, parser)
    }
}

fn parse_text_module<'a>(
    span: wast::Span,
    id: Option<wast::Id<'a>>,
    name: Option<wast::NameAnnotation<'a>>,
    parser: Parser<'a>,
) -> Result<Module<'a>> {
    let _r = parser.register_annotation("custom");
    let _r = parser.register_annotation("interface");

    let mut fields = Vec::new();
    let mut adapters = Vec::new();
    while !parser.is_empty() {
        parser.parens(|parser| {
            if parser.peek::<annotation::interface>() {
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
            id,
            name,
            kind: wast::ModuleKind::Text(fields),
        },
        adapters,
    })
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
        parser.parse::<annotation::interface>()?;
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

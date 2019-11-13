use crate::ast::{self, kw};
use wast::parser::{Parse, Parser, Result};

pub struct Wit<'a> {
    pub module: Module<'a>,
}

impl<'a> Parse<'a> for Wit<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let module = parser.parens(|parser| parser.parse())?;
        Ok(Wit { module })
    }
}

pub struct Module<'a> {
    pub core: wast::Module<'a>,
    pub adapters: Vec<Adapter<'a>>,
}

impl Module<'_> {
    pub fn encode(&mut self) -> std::result::Result<Vec<u8>, wast::Error> {
        let names = self.core.resolve()?;
        crate::resolve::resolve(&mut self.adapters, &names)?;
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

pub enum Adapter<'a> {
    Type(ast::Type<'a>),
    Import(ast::Import<'a>),
    Export(ast::Export<'a>),
    Func(ast::Func<'a>),
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
        Err(l.error())
    }
}

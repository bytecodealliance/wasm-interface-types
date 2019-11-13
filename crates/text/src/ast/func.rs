use crate::ast::{self, kw};
use wast::parser::{Parse, Parser, Result};

pub struct Func<'a> {
    pub span: wast::Span,
    pub name: Option<wast::Id<'a>>,
    pub export: Option<&'a str>,
    pub ty: ast::TypeUse<'a>,
    pub kind: FuncKind<'a>,
}

pub enum FuncKind<'a> {
    Import { module: &'a str, name: &'a str },
    Inline { instrs: Vec<ast::Instruction<'a>> },
}

impl<'a> Parse<'a> for Func<'a> {
    fn parse(parser: Parser<'a>) -> Result<Func<'a>> {
        let span = parser.parse::<kw::func>()?.0;
        let name = parser.parse()?;
        let export = if parser.peek2::<kw::export>() {
            Some(parser.parens(|p| {
                p.parse::<kw::export>()?;
                p.parse()
            })?)
        } else {
            None
        };

        let (ty, kind) = if parser.peek2::<kw::import>() {
            let (module, name) = parser.parens(|p| {
                p.parse::<kw::import>()?;
                Ok((p.parse()?, p.parse()?))
            })?;
            (parser.parse()?, FuncKind::Import { module, name })
        } else {
            let ty = parser.parse()?;
            let mut instrs = Vec::new();
            while !parser.is_empty() {
                instrs.push(parser.parse()?);
            }
            (ty, FuncKind::Inline { instrs })
        };

        Ok(Func {
            span,
            name,
            export,
            ty,
            kind,
        })
    }
}

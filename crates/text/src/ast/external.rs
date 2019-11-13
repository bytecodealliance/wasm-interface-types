use crate::ast::{self, kw};
use wast::parser::{Parse, Parser, Result};

pub struct Import<'a> {
    pub span: wast::Span,
    pub id: Option<wast::Id<'a>>,
    pub module: &'a str,
    pub name: &'a str,
    pub ty: ast::TypeUse<'a>,
}

impl<'a> Parse<'a> for Import<'a> {
    fn parse(parser: Parser<'a>) -> Result<Import<'a>> {
        let span = parser.parse::<kw::import>()?.0;
        let module = parser.parse()?;
        let name = parser.parse()?;
        let (id, ty) = parser.parens(|parser| {
            parser.parse::<kw::func>()?;
            Ok((parser.parse()?, parser.parse()?))
        })?;
        Ok(Import {
            span,
            module,
            name,
            id,
            ty,
        })
    }
}

pub struct Export<'a> {
    pub func: wast::Index<'a>,
    pub name: &'a str,
}

impl<'a> Parse<'a> for Export<'a> {
    fn parse(parser: Parser<'a>) -> Result<Export<'a>> {
        parser.parse::<kw::export>()?;
        let name = parser.parse()?;
        let func = parser.parens(|parser| {
            parser.parse::<kw::func>()?;
            parser.parse()
        })?;
        Ok(Export { name, func })
    }
}

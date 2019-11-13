use crate::ast;
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
        drop(parser);
        panic!()
    }
}

pub struct Export<'a> {
    pub func: wast::Index<'a>,
    pub name: &'a str,
}

impl<'a> Parse<'a> for Export<'a> {
    fn parse(parser: Parser<'a>) -> Result<Export<'a>> {
        drop(parser);
        panic!()
    }
}

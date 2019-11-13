use crate::ast;
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
        drop(parser);
        panic!()
    }
}

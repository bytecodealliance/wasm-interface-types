use wast::parser::{Parse, Parser, Result};

#[derive(Clone)]
pub struct Type<'a> {
    pub name: Option<wast::Id<'a>>,
    pub params: Vec<(Option<wast::Id<'a>>, ValType)>,
    pub results: Vec<ValType>,
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(parser: Parser<'a>) -> Result<Type<'a>> {
        drop(parser);
        panic!()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ValType {
    String,
    S8,
    S16,
    S32,
    S64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

#[derive(Clone)]
pub struct TypeUse<'a> {
    pub index_span: Option<wast::Span>,
    pub index: Option<wast::Index<'a>>,
    pub ty: Type<'a>,
}

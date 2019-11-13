use wast::parser::{Parse, Parser, Result};

pub struct Type<'a> {
    pub name: Option<wast::Id<'a>>,
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(parser: Parser<'a>) -> Result<Type<'a>> {
        drop(parser);
        panic!()
    }
}

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

pub struct TypeUse<'a> {
    pub index: Option<wast::Index<'a>>,
    pub ty: Type<'a>,
}

use crate::ast::kw;
use wast::parser::{Parse, Parser, Result};

#[derive(Clone)]
pub struct Type<'a> {
    pub name: Option<wast::Id<'a>>,
    pub params: Vec<(Option<wast::Id<'a>>, ValType)>,
    pub results: Vec<ValType>,
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(parser: Parser<'a>) -> Result<Type<'a>> {
        parser.parse::<kw::r#type>()?;
        let name = parser.parse()?;

        let mut params = Vec::new();
        let mut results = Vec::new();

        while parser.peek2::<kw::param>() {
            params.push(parser.parens(|p| {
                p.parse::<kw::param>()?;
                let id = p.parse()?;
                let ty = p.parse()?;
                Ok((id, ty))
            })?);
        }
        while parser.peek2::<kw::result>() {
            results.push(parser.parens(|p| {
                p.parse::<kw::result>()?;
                p.parse()
            })?);
        }

        Ok(Type {
            name,
            params,
            results,
        })
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

impl<'a> Parse<'a> for ValType {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::s8>() {
            parser.parse::<kw::s8>()?;
            return Ok(ValType::S8)
        }
        if l.peek::<kw::s16>() {
            parser.parse::<kw::s16>()?;
            return Ok(ValType::S16)
        }
        if l.peek::<kw::s32>() {
            parser.parse::<kw::s32>()?;
            return Ok(ValType::S32)
        }
        if l.peek::<kw::s64>() {
            parser.parse::<kw::s64>()?;
            return Ok(ValType::S64)
        }
        if l.peek::<kw::u8>() {
            parser.parse::<kw::u8>()?;
            return Ok(ValType::U8)
        }
        if l.peek::<kw::u16>() {
            parser.parse::<kw::u16>()?;
            return Ok(ValType::U16)
        }
        if l.peek::<kw::u32>() {
            parser.parse::<kw::u32>()?;
            return Ok(ValType::U32)
        }
        if l.peek::<kw::u64>() {
            parser.parse::<kw::u64>()?;
            return Ok(ValType::U64)
        }
        if l.peek::<kw::f32>() {
            parser.parse::<kw::f32>()?;
            return Ok(ValType::F32)
        }
        if l.peek::<kw::f64>() {
            parser.parse::<kw::f64>()?;
            return Ok(ValType::F64)
        }
        if l.peek::<kw::string>() {
            parser.parse::<kw::string>()?;
            return Ok(ValType::String)
        }
        Err(l.error())
    }
}

#[derive(Clone)]
pub struct TypeUse<'a> {
    pub index_span: Option<wast::Span>,
    pub index: Option<wast::Index<'a>>,
    pub ty: Type<'a>,
}

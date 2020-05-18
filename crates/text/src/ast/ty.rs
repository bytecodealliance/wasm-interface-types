use crate::ast::kw;
use wast::parser::{Parse, Parser, Result};

/// A type declaration in a wasm interface type subsection
#[derive(Clone)]
pub struct Type<'a> {
    /// The optional name of this type, used to refer to it from elsewhere.
    pub name: Option<wast::Id<'a>>,
    /// Explicitly listed parameters with optional names, if any.
    pub params: Vec<(Option<wast::Id<'a>>, ValType)>,
    /// The results of this function signature.
    pub results: Vec<ValType>,
}

fn finish_parse<'a>(
    parser: Parser<'a>,
    params: &mut Vec<(Option<wast::Id<'a>>, ValType)>,
    results: &mut Vec<ValType>,
) -> Result<()> {
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
    Ok(())
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(parser: Parser<'a>) -> Result<Type<'a>> {
        parser.parse::<kw::r#type>()?;
        let name = parser.parse()?;

        let mut params = Vec::new();
        let mut results = Vec::new();
        parser.parens(|p| {
            p.parse::<kw::func>()?;
            finish_parse(parser, &mut params, &mut results)
        })?;

        Ok(Type {
            name,
            params,
            results,
        })
    }
}

/// Possible value types that can be used in function signatures and such.
#[derive(Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ValType {
    String,
    Externref,
    S8,
    S16,
    S32,
    S64,
    I32,
    I64,
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
            return Ok(ValType::S8);
        }
        if l.peek::<kw::s16>() {
            parser.parse::<kw::s16>()?;
            return Ok(ValType::S16);
        }
        if l.peek::<kw::s32>() {
            parser.parse::<kw::s32>()?;
            return Ok(ValType::S32);
        }
        if l.peek::<kw::s64>() {
            parser.parse::<kw::s64>()?;
            return Ok(ValType::S64);
        }
        if l.peek::<kw::u8>() {
            parser.parse::<kw::u8>()?;
            return Ok(ValType::U8);
        }
        if l.peek::<kw::u16>() {
            parser.parse::<kw::u16>()?;
            return Ok(ValType::U16);
        }
        if l.peek::<kw::u32>() {
            parser.parse::<kw::u32>()?;
            return Ok(ValType::U32);
        }
        if l.peek::<kw::u64>() {
            parser.parse::<kw::u64>()?;
            return Ok(ValType::U64);
        }
        if l.peek::<kw::f32>() {
            parser.parse::<kw::f32>()?;
            return Ok(ValType::F32);
        }
        if l.peek::<kw::f64>() {
            parser.parse::<kw::f64>()?;
            return Ok(ValType::F64);
        }
        if l.peek::<kw::i32>() {
            parser.parse::<kw::i32>()?;
            return Ok(ValType::I32);
        }
        if l.peek::<kw::i64>() {
            parser.parse::<kw::i64>()?;
            return Ok(ValType::I64);
        }
        if l.peek::<kw::externref>() {
            parser.parse::<kw::externref>()?;
            return Ok(ValType::Externref);
        }
        if l.peek::<kw::anyref>() {
            parser.parse::<kw::anyref>()?;
            return Ok(ValType::Externref);
        }
        if l.peek::<kw::string>() {
            parser.parse::<kw::string>()?;
            return Ok(ValType::String);
        }
        Err(l.error())
    }
}

/// An inline type definition or a use of a type defined elsewhere.
#[derive(Clone)]
pub struct TypeUse<'a> {
    /// Where the index was defined, if it was defined.
    pub index_span: Option<wast::Span>,
    /// The type declaration that this is reference.
    pub index: Option<wast::Index<'a>>,
    /// The inline parameters, if any.
    pub ty: Type<'a>,
}

impl<'a> Parse<'a> for TypeUse<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let index = if parser.peek2::<kw::r#type>() {
            Some(parser.parens(|parser| {
                parser.parse::<kw::r#type>()?;
                Ok((parser.cur_span(), parser.parse()?))
            })?)
        } else {
            None
        };
        let (index_span, index) = match index {
            Some((a, b)) => (Some(a), Some(b)),
            None => (None, None),
        };
        let mut params = Vec::new();
        let mut results = Vec::new();
        finish_parse(parser, &mut params, &mut results)?;

        Ok(TypeUse {
            index,
            index_span,
            ty: Type {
                name: None,
                params,
                results,
            },
        })
    }
}

use crate::ast::{self, kw};
use wast::parser::{Parse, Parser, Result};

/// A definition of an adapter function
pub struct Func<'a> {
    /// Where this function was defined
    pub span: wast::Span,
    /// An optional name used to refer to this function
    pub name: Option<wast::Id<'a>>,
    /// An optional name that this function is being exported under
    pub export: Option<&'a str>,
    /// The type that this function has
    pub ty: ast::TypeUse<'a>,
    /// How this function is defined
    pub kind: FuncKind<'a>,
}

/// Different flavors of functions that can be defined.
pub enum FuncKind<'a> {
    /// An sugared `import` declaration.
    Import {
        /// Where we're importing from
        module: &'a str,
        /// What we're importing
        name: &'a str,
    },

    /// An inline function definition which contains actual instructions
    #[allow(missing_docs)]
    Inline { instrs: ast::Instructions<'a> },
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
            let instrs = parser.parse()?;
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

/// A list of instructions, possibly in s-expression form
#[allow(missing_docs)]
pub struct Instructions<'a> {
    pub instrs: Vec<ast::Instruction<'a>>,
}

impl<'a> Parse<'a> for Instructions<'a> {
    fn parse(parser: Parser<'a>) -> Result<Instructions<'a>> {
        let mut instrs = Vec::new();
        while !parser.is_empty() {
            instrs.push(parser.parse()?);
        }
        Ok(Instructions { instrs })
    }
}

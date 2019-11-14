use crate::ast::{self, kw};
use wast::parser::{Parse, Parser, Result};

/// A means of implementing a core wasm imported function with an adapter
/// function.
pub struct Implement<'a> {
    /// Where this directive was defined
    pub span: wast::Span,
    /// What's being implemented
    pub implemented: Implemented<'a>,
    /// How it's being implemented
    pub implementation: Implementation<'a>,
}

/// Different ways to specify what's being implemented
pub enum Implemented<'a> {
    /// A specification of what's being impelmented by module/name
    #[allow(missing_docs)]
    ByName { module: &'a str, name: &'a str },

    /// A specification using an index
    ByIndex(wast::Index<'a>),
}

/// Different ways to specify an implementation
pub enum Implementation<'a> {
    /// A specification using an index
    ByIndex(wast::Index<'a>),

    /// An inline definition of what's being implemented
    Inline {
        /// The type that this function has
        ty: ast::TypeUse<'a>,
        /// Body of the implementation
        instrs: ast::Instructions<'a>,
    },
}

impl<'a> Parse<'a> for Implement<'a> {
    fn parse(parser: Parser<'a>) -> Result<Implement<'a>> {
        let span = parser.parse::<kw::implement>()?.0;

        let implemented = parser.parens(|parser| {
            if parser.peek::<kw::func>() {
                parser.parse::<kw::func>()?;
                Ok(Implemented::ByIndex(parser.parse()?))
            } else {
                parser.parse::<kw::import>()?;
                Ok(Implemented::ByName {
                    module: parser.parse()?,
                    name: parser.parse()?,
                })
            }
        })?;

        let implementation = if parser.peek2::<kw::func>() {
            parser.parens(|parser| {
                parser.parse::<kw::func>()?;
                Ok(Implementation::ByIndex(parser.parse()?))
            })?
        } else {
            Implementation::Inline {
                ty: parser.parse()?,
                instrs: parser.parse()?,
            }
        };

        Ok(Implement {
            span,
            implemented,
            implementation,
        })
    }
}

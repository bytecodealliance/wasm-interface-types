use crate::ast::{self, kw};
use wast::parser::{Parse, Parser, Result};

/// An imported function declaration using wasm interface types.
pub struct Import<'a> {
    /// Where this `import` was defined.
    pub span: wast::Span,
    /// The name of this import to refer to
    pub id: Option<wast::Id<'a>>,
    /// Where this was imported from
    pub module: &'a str,
    /// What is being imported
    pub name: &'a str,
    /// The type signature of the function being imported.
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

/// An exported wasm interface types function
pub struct Export<'a> {
    /// The function being exported
    pub func: wast::Index<'a>,
    /// The name we're exporting under
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

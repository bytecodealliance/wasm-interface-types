use wast::parser::{Parse, Parser, Result, Cursor, Peek};

mod external;
mod func;
mod instr;
mod module;
mod ty;
pub use self::external::*;
pub use self::func::*;
pub use self::instr::*;
pub use self::module::*;
pub use self::ty::*;

mod kw {
    pub use wast::kw::*;
}

struct AtInterface(wast::Span);

impl Parse<'_> for AtInterface {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.step(|c| {
            if let Some(("@interface", rest)) = c.reserved() {
                return Ok((AtInterface(c.cur_span()), rest));
            }
            Err(c.error("expected `@interface`"))
        })
    }
}

impl Peek for AtInterface {
    fn peek(cursor: Cursor<'_>) -> bool {
        match cursor.reserved() {
            Some(("@interface", _)) => true,
            _ => false,
        }
    }

    fn display() -> &'static str {
        "@interface"
    }
}

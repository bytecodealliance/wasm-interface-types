use wast::parser::{Cursor, Parse, Parser, Peek, Result};

mod external;
mod func;
mod implement;
mod instr;
mod module;
mod ty;
pub use self::external::*;
pub use self::func::*;
pub use self::implement::*;
pub use self::instr::*;
pub use self::module::*;
pub use self::ty::*;

mod kw {
    pub use wast::kw::*;

    wast::custom_keyword!(implement);
    wast::custom_keyword!(s16);
    wast::custom_keyword!(s32);
    wast::custom_keyword!(s64);
    wast::custom_keyword!(s8);
    wast::custom_keyword!(string);
    wast::custom_keyword!(u16);
    wast::custom_keyword!(u32);
    wast::custom_keyword!(u64);
    wast::custom_keyword!(u8);
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

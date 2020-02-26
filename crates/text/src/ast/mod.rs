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

mod annotation {
    wast::annotation!(interface);
}

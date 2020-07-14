use wast::parser::{Parse, Parser, Result};
use wast::Span;

macro_rules! instructions {
    ($(#[$a:meta])* pub enum Instruction<'a> {
        $(
            $name:ident $(($($arg:tt)*))? : $instr:tt,
        )*
    }) => (
        $(#[$a])*
        pub enum Instruction<'a> {
            $(
                $name $(( $($arg)* ))?,
            )*
        }

        #[allow(non_snake_case)]
        impl<'a> Parse<'a> for Instruction<'a> {
            fn parse(parser: Parser<'a>) -> Result<Self> {
                $(
                    fn $name<'a>(_parser: Parser<'a>) -> Result<Instruction<'a>> {
                        Ok(Instruction::$name $((
                            _parser.parse::<$($arg)*>()?,
                        ))?)
                    }
                )*
                let parse_remainder = parser.step(|c| {
                    let (kw, rest) = match c.keyword() {
                        Some(pair) => pair,
                        None => return Err(c.error("expected an instruction")),
                    };
                    match kw {
                        $($instr => Ok(($name as fn(_) -> _, rest)),)*
                        _ => return Err(c.error("unknown operator or unexpected token")),
                    }
                })?;
                parse_remainder(parser)
            }
        }
    );

    (@first $first:ident $($t:tt)*) => ($first);
}

instructions! {
    /// List of instructions in adapter functions.
    #[allow(missing_docs)]
    pub enum Instruction<'a> {
        ArgGet(wast::Index<'a>) : "arg.get",
        CallCore(wast::Index<'a>) : "call-core",
        MemoryToString(MemoryToString<'a>) : "memory-to-string",
        StringToMemory(StringToMemory<'a>) : "string-to-memory",
        CallAdapter(wast::Index<'a>) : "call-adapter",
        DeferCallCore(wast::Index<'a>) : "defer-call-core",

        I32ToS8 : "i32-to-s8",
        I32ToS8X : "i32-to-s8x",
        I32ToU8 : "i32-to-u8",
        I32ToS16 : "i32-to-s16",
        I32ToS16X : "i32-to-s16x",
        I32ToU16 : "i32-to-u16",
        I32ToS32 : "i32-to-s32",
        I32ToU32 : "i32-to-u32",
        I32ToS64 : "i32-to-s64",
        I32ToU64 : "i32-to-u64",

        I64ToS8 : "i64-to-s8",
        I64ToS8X : "i64-to-s8x",
        I64ToU8 : "i64-to-u8",
        I64ToS16 : "i64-to-s16",
        I64ToS16X : "i64-to-s16x",
        I64ToU16 : "i64-to-u16",
        I64ToS32 : "i64-to-s32",
        I64ToS32X : "i64-to-s32x",
        I64ToU32 : "i64-to-u32",
        I64ToS64 : "i64-to-s64",
        I64ToU64 : "i64-to-u64",

        S8ToI32 : "s8-to-i32",
        U8ToI32 : "u8-to-i32",
        S16ToI32 : "s16-to-i32",
        U16ToI32 : "u16-to-i32",
        S32ToI32 : "s32-to-i32",
        U32ToI32 : "u32-to-i32",
        S64ToI32 : "s64-to-i32",
        S64ToI32X : "s64-to-i32x",
        U64ToI32 : "u64-to-i32",
        U64ToI32X : "u64-to-i32x",

        S8ToI64 : "s8-to-i64",
        U8ToI64 : "u8-to-i64",
        S16ToI64 : "s16-to-i64",
        U16ToI64 : "u16-to-i64",
        S32ToI64 : "s32-to-i64",
        U32ToI64 : "u32-to-i64",
        S64ToI64 : "s64-to-i64",
        U64ToI64 : "u64-to-i64",
    }
}

/// Payload of the `memory-to-string` instruction
pub struct MemoryToString<'a> {
    /// Index of the memory that the string is coming from.
    pub mem: wast::Index<'a>,
}

impl<'a> Parse<'a> for MemoryToString<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        Ok(MemoryToString {
            mem: parser
                .parse::<Option<_>>()?
                .unwrap_or(wast::Index::Num(0, Span::from_offset(0))),
        })
    }
}

/// Payload of the `string-to-memory` instruction
pub struct StringToMemory<'a> {
    /// Function which is used as a memory allocator to allocate memory to place
    /// the string in `mem`.
    pub malloc: wast::Index<'a>,
    /// Index of the memory that the string is coming from.
    pub mem: wast::Index<'a>,
}

impl<'a> Parse<'a> for StringToMemory<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        Ok(StringToMemory {
            malloc: parser.parse()?,
            mem: parser
                .parse::<Option<_>>()?
                .unwrap_or(wast::Index::Num(0, Span::from_offset(0))),
        })
    }
}

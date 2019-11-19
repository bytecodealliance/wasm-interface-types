use wast::parser::{Parse, Parser, Result};

macro_rules! instructions {
    ($(#[$a:meta])* pub enum Instruction<'a> {
        $(
            $name:ident $(($($arg:tt)*))? : [$($binary:tt)*] : $instr:tt,
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

        impl crate::binary::Encode for Instruction<'_> {
            #[allow(non_snake_case)]
            fn encode(&self, v: &mut Vec<u8>) {
                match self {
                    $(
                        Instruction::$name $((instructions!(@first $($arg)*)))? => {
                            fn encode<'a>($(arg: &$($arg)*,)? v: &mut Vec<u8>) {
                                v.extend_from_slice(&[$($binary)*]);
                                $(<$($arg)* as crate::binary::Encode>::encode(arg, v);)?
                            }
                            encode($( instructions!(@first $($arg)*), )? v)
                        }
                    )*
                }
            }
        }
    );

    (@first $first:ident $($t:tt)*) => ($first);
}

instructions! {
    /// List of instructions in adapter functions.
    #[allow(missing_docs)]
    pub enum Instruction<'a> {
        ArgGet(wast::Index<'a>) : [0x00] : "arg.get",
        CallCore(wast::Index<'a>) : [0x01] : "call-core",
        End : [0x02] : "end",
        MemoryToString(MemoryToString<'a>) : [0x03] : "memory-to-string",
        StringToMemory(StringToMemory<'a>) : [0x04] : "string-to-memory",
        CallAdapter(wast::Index<'a>) : [0x05] : "call-adapter",
        DeferCallCore(wast::Index<'a>) : [0x06] : "defer-call-core",

        I32ToS8 : [0x07] : "i32-to-s8",
        I32ToS8X : [0x08] : "i32-to-s8x",
        I32ToU8 : [0x09] : "i32-to-u8",
        I32ToS16 : [0x0a] : "i32-to-s16",
        I32ToS16X : [0x0b] : "i32-to-s16x",
        I32ToU16 : [0x0c] : "i32-to-u16",
        I32ToS32 : [0x0d] : "i32-to-s32",
        I32ToU32 : [0x0e] : "i32-to-u32",
        I32ToS64 : [0x0f] : "i32-to-s64",
        I32ToU64 : [0x10] : "i32-to-u64",

        I64ToS8 : [0x11] : "i64-to-s8",
        I64ToS8X : [0x12] : "i64-to-s8x",
        I64ToU8 : [0x13] : "i64-to-u8",
        I64ToS16 : [0x14] : "i64-to-s16",
        I64ToS16X : [0x15] : "i64-to-s16x",
        I64ToU16 : [0x16] : "i64-to-u16",
        I64ToS32 : [0x17] : "i64-to-s32",
        I64ToS32X : [0x18] : "i64-to-s32x",
        I64ToU32 : [0x19] : "i64-to-u32",
        I64ToS64 : [0x1a] : "i64-to-s64",
        I64ToU64 : [0x1b] : "i64-to-u64",

        S8ToI32 : [0x1c] : "s8-to-i32",
        U8ToI32 : [0x1d] : "u8-to-i32",
        S16ToI32 : [0x1e] : "s16-to-i32",
        U16ToI32 : [0x1f] : "u16-to-i32",
        S32ToI32 : [0x20] : "s32-to-i32",
        U32ToI32 : [0x21] : "u32-to-i32",
        S64ToI32 : [0x22] : "s64-to-i32",
        S64ToI32X : [0x23] : "s64-to-i32x",
        U64ToI32 : [0x24] : "u64-to-i32",
        U64ToI32X : [0x25] : "u64-to-i32x",

        S8ToI64 : [0x26] : "s8-to-i64",
        U8ToI64 : [0x27] : "u8-to-i64",
        S16ToI64 : [0x28] : "s16-to-i64",
        U16ToI64 : [0x29] : "u16-to-i64",
        S32ToI64 : [0x2a] : "s32-to-i64",
        U32ToI64 : [0x2b] : "u32-to-i64",
        S64ToI64 : [0x2c] : "s64-to-i64",
        U64ToI64 : [0x2d] : "u64-to-i64",
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
            mem: parser.parse::<Option<_>>()?.unwrap_or(wast::Index::Num(0)),
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
            mem: parser.parse::<Option<_>>()?.unwrap_or(wast::Index::Num(0)),
        })
    }
}

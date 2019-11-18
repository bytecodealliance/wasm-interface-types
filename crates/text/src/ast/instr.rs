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

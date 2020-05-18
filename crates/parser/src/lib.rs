//! A parser for the WebAssembly Interface Types binary section.
//!
//! This crates is intended to provide the low-level parsing support for
//! WebAssembly Interface Types. The official specification has no mention of
//! the binary encoding, so this is currently an invented format. This will only
//! work when paired with similar tools using the same format, so this is
//! unstable and you'll need to be careful about using this. This is not an
//! official project nor is it a standard, treat it appropriately!

#![deny(missing_docs)]

use std::fmt;
use std::str;

/// Top-level parser for the WebAssembly Interface Types binary section.
///
/// This `Parser` is used to iterate over [`Section`] instances to learn about
/// each section in the binary format.
#[derive(Clone)]
pub struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

/// Errors that can happen during parsing.
#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorInner>,
}

/// A convenience typedef with `Error` as the default error.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
struct ErrorInner {
    at: usize,
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    InvalidVersion(String),
    UlebTooBig(u64),
    UlebInvalid,
    UnexpectedEof,
    InvalidUtf8,
    InvalidSection(u8),
    InvalidValType(u8),
    InvalidInstruction(u8),
    Expected(usize),
    TrailingBytes,
}

impl<'a> Parser<'a> {
    /// Creates a new [`Parser`] from the given binary blob.
    ///
    /// Currently the binary blob is expected to be the payload of a custom
    /// section called `wasm-interface-types`. This will almost surely change in
    /// the future.
    ///
    /// The `offset` argument is the offset in which `bytes` was found in the
    /// original binary file, used to generate error messages.
    pub fn new(offset: usize, bytes: &'a [u8]) -> Result<Parser<'a>> {
        let mut parser = Parser { bytes, pos: offset };
        let version = <&str as Parse>::parse(&mut parser)?;
        if version != wit_schema_version::VERSION {
            parser.pos = 0;
            return Err(parser.error(ErrorKind::InvalidVersion(version.to_string())));
        }
        Ok(parser)
    }

    /// Returns if there are no more bytes to parse in this `Parser`, and all
    /// sections have been read.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Attempts to parse the next [`Section`] from this [`Parser`].
    ///
    /// Returns an error if this parser cannot be advanced to parse another
    /// section. Note that the section itself will not be fully parsed, but
    /// rather it will return a placeholder which be further used to parse the
    /// contents of the section.
    pub fn section(&mut self) -> Result<Section<'a>> {
        self.parse()
    }

    fn parse<T: Parse<'a>>(&mut self) -> Result<T> {
        T::parse(self)
    }

    fn parse_next_in_section<T: Parse<'a>>(&mut self, cnt: &mut u32) -> Option<Result<T>> {
        if *cnt == 0 {
            if self.bytes.len() != 0 {
                return Some(Err(self.error(ErrorKind::TrailingBytes)));
            }
            None
        } else {
            *cnt -= 1;
            Some(T::parse(self))
        }
    }

    fn error(&self, kind: ErrorKind) -> Error {
        Error {
            inner: Box::new(ErrorInner { at: self.pos, kind }),
        }
    }
}

trait Parse<'a>: Sized {
    fn parse(parser: &mut Parser<'a>) -> Result<Self>;
}

/// List of various sections that can be found in the WebAssembly Interface
/// Types section.
#[allow(missing_docs)]
pub enum Section<'a> {
    Type(Types<'a>),
    Import(Imports<'a>),
    Export(Exports<'a>),
    Func(Funcs<'a>),
    Implement(Implements<'a>),
}

impl<'a> Parse<'a> for Section<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let id_pos = parser.pos;
        let id = u8::parse(parser)?;
        let bytes = <&[u8]>::parse(parser)?;
        let mut parser = Parser {
            bytes,
            pos: parser.pos - bytes.len(),
        };
        match id {
            0 => {
                let cnt = parser.parse()?;
                Ok(Section::Type(Types { parser, cnt }))
            }
            1 => {
                let cnt = parser.parse()?;
                Ok(Section::Import(Imports { parser, cnt }))
            }
            2 => {
                let cnt = parser.parse()?;
                Ok(Section::Func(Funcs { parser, cnt }))
            }
            3 => {
                let cnt = parser.parse()?;
                Ok(Section::Export(Exports { parser, cnt }))
            }
            4 => {
                let cnt = parser.parse()?;
                Ok(Section::Implement(Implements { parser, cnt }))
            }
            n => {
                parser.pos = id_pos;
                Err(parser.error(ErrorKind::InvalidSection(n)))
            }
        }
    }
}

impl<'a> Parse<'a> for u8 {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.bytes.get(0).cloned() {
            Some(byte) => {
                parser.pos += 1;
                parser.bytes = &parser.bytes[1..];
                Ok(byte)
            }
            None => Err(parser.error(ErrorKind::UnexpectedEof)),
        }
    }
}

impl<'a> Parse<'a> for &'a [u8] {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let len = parser.parse::<u32>()? as usize;
        match parser.bytes.get(..len) {
            Some(n) => {
                parser.pos += len;
                parser.bytes = &parser.bytes[len..];
                Ok(n)
            }
            None => Err(parser.error(ErrorKind::Expected(len))),
        }
    }
}

impl<'a> Parse<'a> for &'a str {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let pos = parser.pos;
        match str::from_utf8(parser.parse()?) {
            Ok(s) => Ok(s),
            Err(_) => {
                parser.pos = pos;
                Err(parser.error(ErrorKind::InvalidUtf8))
            }
        }
    }
}

impl<'a> Parse<'a> for u32 {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut bytes = parser.bytes;
        match leb128::read::unsigned(&mut bytes) {
            Ok(n) if n <= u32::max_value() as u64 => {
                parser.pos += parser.bytes.len() - bytes.len();
                parser.bytes = bytes;
                Ok(n as u32)
            }
            Ok(n) => Err(parser.error(ErrorKind::UlebTooBig(n))),
            Err(_) => Err(parser.error(ErrorKind::UlebInvalid)),
        }
    }
}

/// An iterator over instances of [`Type`] in the type subsection of a wasm
/// interface types section.
pub struct Types<'a> {
    parser: Parser<'a>,
    cnt: u32,
}

impl<'a> Iterator for Types<'a> {
    type Item = Result<Type>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.parse_next_in_section(&mut self.cnt)
    }
}

/// A type signatured reference by imports/functions, very similar to a wasm
/// type declaration.
pub struct Type {
    /// Each of the parameter value types of this type signature.
    pub params: Vec<ValType>,
    /// Each of the returned value types of this type signature.
    pub results: Vec<ValType>,
}

impl<'a> Parse<'a> for Type {
    fn parse(parser: &mut Parser<'a>) -> Result<Type> {
        let mut types = || -> Result<Vec<ValType>> {
            let cnt = parser.parse::<u32>()?;
            (0..cnt).map(|_| parser.parse()).collect()
        };
        Ok(Type {
            params: types()?,
            results: types()?,
        })
    }
}

/// List of value types supported in wasm interface types
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValType {
    S8,
    S16,
    S32,
    S64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    String,
    Externref,
    I32,
    I64,
}

impl<'a> Parse<'a> for ValType {
    fn parse(parser: &mut Parser<'a>) -> Result<ValType> {
        Ok(match parser.parse::<u8>()? {
            0 => ValType::S8,
            1 => ValType::S16,
            2 => ValType::S32,
            3 => ValType::S64,
            4 => ValType::U8,
            5 => ValType::U16,
            6 => ValType::U32,
            7 => ValType::U64,
            8 => ValType::F32,
            9 => ValType::F64,
            10 => ValType::String,
            11 => ValType::Externref,
            12 => ValType::I32,
            13 => ValType::I64,
            n => return Err(parser.error(ErrorKind::InvalidValType(n))),
        })
    }
}

/// An iterator over instances of [`Import`] in the import subsection of a wasm
/// interface types section.
pub struct Imports<'a> {
    parser: Parser<'a>,
    cnt: u32,
}

impl<'a> Iterator for Imports<'a> {
    type Item = Result<Import<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.parse_next_in_section(&mut self.cnt)
    }
}

/// An element of the [`Imports`] subsection which lists where the import comes
/// from as well as the type signature of the function.
///
/// Currently imports are always functions.
pub struct Import<'a> {
    /// The wasm module name where this import comes from.
    pub module: &'a str,
    /// The name of the field from the wasm module that this import comes from.
    pub name: &'a str,
    /// The type signature of the function that this import represents.
    pub ty: u32,
}

impl<'a> Parse<'a> for Import<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Import<'a>> {
        Ok(Import {
            module: parser.parse()?,
            name: parser.parse()?,
            ty: parser.parse()?,
        })
    }
}

/// An iterator over instances of [`Export`] in the export subsection of a wasm
/// interface types section.
pub struct Exports<'a> {
    parser: Parser<'a>,
    cnt: u32,
}

impl<'a> Iterator for Exports<'a> {
    type Item = Result<Export<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.parse_next_in_section(&mut self.cnt)
    }
}

/// An element of the [`Exports`] subsection which lists all exports from wasm
/// interface types, used to export interface adapters from a module.
pub struct Export<'a> {
    /// The wasm interface types function that this export is referencing.
    pub func: u32,
    /// The name that this export is known by.
    pub name: &'a str,
}

impl<'a> Parse<'a> for Export<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Export<'a>> {
        Ok(Export {
            func: parser.parse()?,
            name: parser.parse()?,
        })
    }
}

/// An iterator over instances of [`Implement`] in the implement subsection of
/// a wasm interface types section.
pub struct Implements<'a> {
    parser: Parser<'a>,
    cnt: u32,
}

impl<'a> Iterator for Implements<'a> {
    type Item = Result<Implement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.parse_next_in_section(&mut self.cnt)
    }
}

/// An element of the [`Implements`] subsection which is a mapping that connects
/// a core wasm func to being implemented by an adapter function.
pub struct Implement {
    /// The wasm core function which is an import that we're implementing.
    pub core_func: u32,
    /// The adapter function that we're implementing with.
    pub adapter_func: u32,
}

impl<'a> Parse<'a> for Implement {
    fn parse(parser: &mut Parser<'a>) -> Result<Implement> {
        Ok(Implement {
            core_func: parser.parse()?,
            adapter_func: parser.parse()?,
        })
    }
}

/// An iterator over instances of [`Func`] in the function subsection of a wasm
/// interface types section.
pub struct Funcs<'a> {
    parser: Parser<'a>,
    cnt: u32,
}

impl<'a> Iterator for Funcs<'a> {
    type Item = Result<Func<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.parse_next_in_section(&mut self.cnt)
    }
}

/// A element of the [`Funcs`] subsection which represents the body of an
/// interface adapter, working with wasm interface types rather than core wasm
/// types.
pub struct Func<'a> {
    /// The wasm interface type signature of this function.
    pub ty: u32,
    parser: Parser<'a>,
}

impl<'a> Parse<'a> for Func<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Func<'a>> {
        let bytes = parser.parse::<&[u8]>()?;
        let mut parser = Parser {
            bytes,
            pos: parser.pos - bytes.len(),
        };
        Ok(Func {
            ty: parser.parse()?,
            parser,
        })
    }
}

impl<'a> Func<'a> {
    /// Returns a parser for the instructions of this function.
    pub fn instrs(&self) -> Instructions<'a> {
        Instructions {
            parser: self.parser.clone(),
        }
    }
}

/// A parser for each [`Instruction`] contained within a [`Func`]
pub struct Instructions<'a> {
    parser: Parser<'a>,
}

impl<'a> Iterator for Instructions<'a> {
    type Item = Result<Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.parser.parse() {
            Ok(Instruction::End) => {
                if self.parser.is_empty() {
                    None
                } else {
                    Some(Err(self.parser.error(ErrorKind::TrailingBytes)))
                }
            }
            other => Some(other),
        }
    }
}

macro_rules! instructions {
    (pub enum Instruction {
        $(
            $name:ident $(($($arg:tt)*))? = $binary:tt,
        )*
    }) => (
        /// Operators that can be found in the body of a function in a wasm
        /// interface types section.
        #[allow(missing_docs)]
        #[derive(Debug)]
        pub enum Instruction {
            $(
                $name $(( $($arg)* ))?,
            )*
        }

        #[allow(non_snake_case)]
        impl<'a> Parse<'a> for Instruction {
            fn parse(parser: &mut Parser<'a>) -> Result<Self> {
                $(
                    fn $name(_parser: &mut Parser<'_>) -> Result<Instruction> {
                        Ok(Instruction::$name $((
                            _parser.parse::<$($arg)*>()?,
                        ))?)
                    }
                )*
                let pos = parser.pos;
                match parser.parse::<u8>()? {
                    $(
                        $binary => $name(parser),
                    )*
                    n => {
                        parser.pos = pos;
                        Err(parser.error(ErrorKind::InvalidInstruction(n)))
                    }
                }
            }
        }
    );
}

instructions! {
    pub enum Instruction {
        ArgGet(u32) = 0x00,
        CallCore(u32) = 0x01,
        End = 0x02,
        MemoryToString(u32) = 0x03,
        StringToMemory(StringToMemory) = 0x04,
        CallAdapter(u32) = 0x05,
        DeferCallCore(u32) = 0x06,

        I32ToS8 = 0x07,
        I32ToS8X = 0x08,
        I32ToU8 = 0x09,
        I32ToS16 = 0x0a,
        I32ToS16X = 0x0b,
        I32ToU16 = 0x0c,
        I32ToS32 = 0x0d,
        I32ToU32 = 0x0e,
        I32ToS64 = 0x0f,
        I32ToU64 = 0x10,

        I64ToS8 = 0x11,
        I64ToS8X = 0x12,
        I64ToU8 = 0x13,
        I64ToS16 = 0x14,
        I64ToS16X = 0x15,
        I64ToU16 = 0x16,
        I64ToS32 = 0x17,
        I64ToS32X = 0x18,
        I64ToU32 = 0x19,
        I64ToS64 = 0x1a,
        I64ToU64 = 0x1b,

        S8ToI32 = 0x1c,
        U8ToI32 = 0x1d,
        S16ToI32 = 0x1e,
        U16ToI32 = 0x1f,
        S32ToI32 = 0x20,
        U32ToI32 = 0x21,
        S64ToI32 = 0x22,
        S64ToI32X = 0x23,
        U64ToI32 = 0x24,
        U64ToI32X = 0x25,

        S8ToI64 = 0x26,
        U8ToI64 = 0x27,
        S16ToI64 = 0x28,
        U16ToI64 = 0x29,
        S32ToI64 = 0x2a,
        U32ToI64 = 0x2b,
        S64ToI64 = 0x2c,
        U64ToI64 = 0x2d,
    }
}

/// Payload of the `string-to-memory` instruction
#[derive(Debug)]
pub struct StringToMemory {
    /// Function in the core module being used to allocate memory in `mem` to
    /// place a string into. Must take one `i32` parameter and return one `i32`
    /// parameter.
    pub malloc: u32,
    /// Memory index that the string will be placed into.
    pub mem: u32,
}

impl<'a> Parse<'a> for StringToMemory {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(StringToMemory {
            malloc: parser.parse()?,
            mem: parser.parse()?,
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse at byte {}: ", self.inner.at)?;
        match &self.inner.kind {
            ErrorKind::InvalidVersion(s) => write!(
                f,
                "schema version `{}` doesn't match `{}`",
                s,
                wit_schema_version::VERSION
            ),
            ErrorKind::UlebTooBig(_) => write!(f, "uleb encoded integer too big"),
            ErrorKind::UlebInvalid => write!(f, "failed to parse uleb integer"),
            ErrorKind::UnexpectedEof => write!(f, "unexpected end-of-file"),
            ErrorKind::InvalidUtf8 => write!(f, "invalid utf-8 string"),
            ErrorKind::InvalidSection(n) => write!(f, "invalid section id: {}", n),
            ErrorKind::InvalidValType(n) => write!(f, "invalid value type: {}", n),
            ErrorKind::InvalidInstruction(n) => write!(f, "invalid instruction: 0x{:02x}", n),
            ErrorKind::Expected(n) => write!(f, "expected {} more bytes but hit eof", n),
            ErrorKind::TrailingBytes => write!(f, "trailing bytes at the end of the section"),
        }
    }
}

impl std::error::Error for Error {}

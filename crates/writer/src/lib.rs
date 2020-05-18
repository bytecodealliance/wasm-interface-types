//! A crate to write the raw wasm interface types section.

#![deny(missing_docs)]

use std::mem;

/// A structure used to write out the raw representation of a wasm interface
/// types subsection.
///
/// This type performs no validation as items are written, but if you're just
/// calling the methods of this writer you should always produce a syntatically
/// valid section at least.
pub struct Writer {
    dst: Vec<u8>,
    tmp: Vec<Vec<u8>>,
}

impl Writer {
    /// Returns a new `Writer` ready for encoding a wasm interface types section.
    pub fn new() -> Writer {
        let mut w = Writer {
            dst: Vec::new(),
            tmp: Vec::new(),
        };
        wit_schema_version::VERSION.encode(&mut w.dst);
        return w;
    }

    /// Returns a section writer used to write out the type subsection of a
    /// wasm interface types section.
    pub fn types(&mut self, cnt: u32) -> Types<'_> {
        Types {
            tmp: self.start_section(cnt),
            dst: self,
        }
    }

    /// Returns a section writer used to write out the import subsection of a
    /// wasm interface types section.
    pub fn imports(&mut self, cnt: u32) -> Imports<'_> {
        Imports {
            tmp: self.start_section(cnt),
            dst: self,
        }
    }

    /// Returns a section writer used to write out the function subsection of a
    /// wasm interface types section.
    pub fn funcs(&mut self, cnt: u32) -> Funcs<'_> {
        Funcs {
            tmp: self.start_section(cnt),
            dst: self,
        }
    }

    /// Returns a section writer used to write out the export subsection of a
    /// wasm interface types section.
    pub fn exports(&mut self, cnt: u32) -> Exports<'_> {
        Exports {
            tmp: self.start_section(cnt),
            dst: self,
        }
    }

    /// Returns a section writer used to write out the implements subsection of a
    /// wasm interface types section.
    pub fn implements(&mut self, cnt: u32) -> Implements<'_> {
        Implements {
            tmp: self.start_section(cnt),
            dst: self,
        }
    }

    /// Consumes this writer, returning all bytes written so far.
    ///
    /// This will only return the payload of the wasm interface types custom
    /// section, not the custom section headers.
    pub fn into_payload(self) -> Vec<u8> {
        self.dst
    }

    /// Consumes this writer, returning all bytes written so far.
    pub fn into_custom_section(mut self) -> Vec<u8> {
        let mut tmp = self.pop_tmp();
        wit_schema_version::SECTION_NAME.encode(&mut tmp);
        tmp.extend_from_slice(&self.dst);
        let mut tmp2 = self.pop_tmp();
        tmp2.push(0);
        tmp.encode(&mut tmp2);
        return tmp2;
    }

    fn pop_tmp(&mut self) -> Vec<u8> {
        self.tmp.pop().unwrap_or(Vec::new())
    }

    fn push_tmp(&mut self, mut tmp: Vec<u8>) {
        tmp.truncate(0);
        self.tmp.push(tmp);
    }

    fn start_section(&mut self, cnt: u32) -> Vec<u8> {
        let mut buf = self.pop_tmp();
        if cnt > 0 {
            cnt.encode(&mut buf);
        }
        buf
    }

    fn finish_section(&mut self, id: u8, data: Vec<u8>) {
        if data.len() > 0 {
            self.dst.push(id);
            data.encode(&mut self.dst);
        }
        self.push_tmp(data);
    }
}

/// Writer for the list of types in a type subsection.
pub struct Types<'a> {
    dst: &'a mut Writer,
    tmp: Vec<u8>,
}

impl Types<'_> {
    /// Adds a new type in this type section.
    ///
    /// The `nparams` argument specifies how many types will be written by the
    /// `params` closure, and the `nresults argument lists how many times the
    /// `results` closure will write out a type.
    ///
    /// # Panics
    ///
    /// Panics if `params` or `results` doesn't write out the same number of
    /// types that are specified.
    pub fn add(
        &mut self,
        nparams: u32,
        params: impl FnOnce(&mut Type<'_>),
        nresults: u32,
        results: impl FnOnce(&mut Type<'_>),
    ) {
        let mut t = Type {
            dst: &mut self.tmp,
            cnt: 0,
        };
        nparams.encode(t.dst);
        params(&mut t);
        assert_eq!(nparams, t.cnt);
        t.cnt = 0;
        nresults.encode(t.dst);
        results(&mut t);
        assert_eq!(nresults, t.cnt);
    }
}

impl Drop for Types<'_> {
    fn drop(&mut self) {
        self.dst
            .finish_section(0x00, mem::replace(&mut self.tmp, Vec::new()));
    }
}

/// A writer to write out a type, or a sequence of types.
pub struct Type<'a> {
    cnt: u32,
    dst: &'a mut Vec<u8>,
}

#[allow(missing_docs)]
#[rustfmt::skip]
impl Type<'_> {
    fn ty(&mut self, byte: u8) {
        self.cnt += 1;
        self.dst.push(byte);
    }

    pub fn s8(&mut self) { self.ty(0x00) }
    pub fn s16(&mut self) { self.ty(0x01) }
    pub fn s32(&mut self) { self.ty(0x02) }
    pub fn s64(&mut self) { self.ty(0x03) }
    pub fn u8(&mut self) { self.ty(0x04) }
    pub fn u16(&mut self) { self.ty(0x05) }
    pub fn u32(&mut self) { self.ty(0x06) }
    pub fn u64(&mut self) { self.ty(0x07) }
    pub fn f32(&mut self) { self.ty(0x08) }
    pub fn f64(&mut self) { self.ty(0x09) }
    pub fn string(&mut self) { self.ty(0x0a) }
    pub fn externref(&mut self) { self.ty(0x0b) }
    pub fn i32(&mut self) { self.ty(0x0c) }
    pub fn i64(&mut self) { self.ty(0x0d) }
}

/// Writer for the list of imports in an import subsection.
pub struct Imports<'a> {
    dst: &'a mut Writer,
    tmp: Vec<u8>,
}

impl Imports<'_> {
    /// Adds a new import in this type section.
    pub fn add(&mut self, module: &str, name: &str, ty: u32) {
        module.encode(&mut self.tmp);
        name.encode(&mut self.tmp);
        ty.encode(&mut self.tmp);
    }
}

impl Drop for Imports<'_> {
    fn drop(&mut self) {
        self.dst
            .finish_section(0x01, mem::replace(&mut self.tmp, Vec::new()));
    }
}

/// Writer for the list of functions in an function subsection.
pub struct Funcs<'a> {
    dst: &'a mut Writer,
    tmp: Vec<u8>,
}

impl<'a> Funcs<'a> {
    /// Adds a new function in this type section.
    pub fn add(&mut self, ty: u32) -> Instructions<'a, '_> {
        let mut dst = self.dst.pop_tmp();
        ty.encode(&mut dst);
        Instructions {
            tmp: dst,
            funcs: self,
        }
    }
}

impl Drop for Funcs<'_> {
    fn drop(&mut self) {
        self.dst
            .finish_section(0x02, mem::replace(&mut self.tmp, Vec::new()));
    }
}

/// Writer for a sequence of instructions in a function subsection
pub struct Instructions<'a, 'b> {
    tmp: Vec<u8>,
    funcs: &'b mut Funcs<'a>,
}

#[allow(missing_docs)]
#[rustfmt::skip]
impl Instructions<'_, '_> {
    pub fn arg_get(&mut self, arg: u32) {
        self.tmp.push(0x00);
        arg.encode(&mut self.tmp);
    }

    pub fn call_core(&mut self, func: u32) {
        self.tmp.push(0x01);
        func.encode(&mut self.tmp);
    }

    pub fn memory_to_string(&mut self, mem: u32) {
        self.tmp.push(0x03);
        mem.encode(&mut self.tmp);
    }

    pub fn string_to_memory(&mut self, malloc: u32, mem: u32) {
        self.tmp.push(0x04);
        malloc.encode(&mut self.tmp);
        mem.encode(&mut self.tmp);
    }

    pub fn call_adapter(&mut self, func: u32) {
        self.tmp.push(0x05);
        func.encode(&mut self.tmp);
    }

    pub fn defer_call_core(&mut self, func: u32) {
        self.tmp.push(0x06);
        func.encode(&mut self.tmp);
    }

    pub fn i32_to_s8(&mut self) { self.tmp.push(0x07) }
    pub fn i32_to_s8x(&mut self) { self.tmp.push(0x08) }
    pub fn i32_to_u8(&mut self) { self.tmp.push(0x09) }
    pub fn i32_to_s16(&mut self) { self.tmp.push(0x0a) }
    pub fn i32_to_s16x(&mut self) { self.tmp.push(0x0b) }
    pub fn i32_to_u16(&mut self) { self.tmp.push(0x0c) }
    pub fn i32_to_s32(&mut self) { self.tmp.push(0x0d) }
    pub fn i32_to_u32(&mut self) { self.tmp.push(0x0e) }
    pub fn i32_to_s64(&mut self) { self.tmp.push(0x0f) }
    pub fn i32_to_u64(&mut self) { self.tmp.push(0x10) }

    pub fn i64_to_s8(&mut self) { self.tmp.push(0x11) }
    pub fn i64_to_s8x(&mut self) { self.tmp.push(0x12) }
    pub fn i64_to_u8(&mut self) { self.tmp.push(0x13) }
    pub fn i64_to_s16(&mut self) { self.tmp.push(0x14) }
    pub fn i64_to_s16x(&mut self) { self.tmp.push(0x15) }
    pub fn i64_to_u16(&mut self) { self.tmp.push(0x16) }
    pub fn i64_to_s32(&mut self) { self.tmp.push(0x17) }
    pub fn i64_to_s32x(&mut self) { self.tmp.push(0x18) }
    pub fn i64_to_u32(&mut self) { self.tmp.push(0x19) }
    pub fn i64_to_s64(&mut self) { self.tmp.push(0x1a) }
    pub fn i64_to_u64(&mut self) { self.tmp.push(0x1b) }

    pub fn s8_to_i32(&mut self) { self.tmp.push(0x1c) }
    pub fn u8_to_i32(&mut self) { self.tmp.push(0x1d) }
    pub fn s16_to_i32(&mut self) { self.tmp.push(0x1e) }
    pub fn u16_to_i32(&mut self) { self.tmp.push(0x1f) }
    pub fn s32_to_i32(&mut self) { self.tmp.push(0x20) }
    pub fn u32_to_i32(&mut self) { self.tmp.push(0x21) }
    pub fn s64_to_i32(&mut self) { self.tmp.push(0x22) }
    pub fn s64_to_i32x(&mut self) { self.tmp.push(0x23) }
    pub fn u64_to_i32(&mut self) { self.tmp.push(0x24) }
    pub fn u64_to_i32x(&mut self) { self.tmp.push(0x25) }

    pub fn s8_to_i64(&mut self) { self.tmp.push(0x26) }
    pub fn u8_to_i64(&mut self) { self.tmp.push(0x27) }
    pub fn s16_to_i64(&mut self) { self.tmp.push(0x28) }
    pub fn u16_to_i64(&mut self) { self.tmp.push(0x29) }
    pub fn s32_to_i64(&mut self) { self.tmp.push(0x2a) }
    pub fn u32_to_i64(&mut self) { self.tmp.push(0x2b) }
    pub fn s64_to_i64(&mut self) { self.tmp.push(0x2c) }
    pub fn u64_to_i64(&mut self) { self.tmp.push(0x2d) }
}

impl Drop for Instructions<'_, '_> {
    fn drop(&mut self) {
        self.tmp.push(0x02);
        let buf = mem::replace(&mut self.tmp, Vec::new());
        buf.encode(&mut self.funcs.tmp);
        self.funcs.dst.push_tmp(buf);
    }
}

/// Writer for the list of exports in an export subsection.
pub struct Exports<'a> {
    dst: &'a mut Writer,
    tmp: Vec<u8>,
}

impl Exports<'_> {
    /// Adds a new export in this type section.
    pub fn add(&mut self, name: &str, func: u32) {
        func.encode(&mut self.tmp);
        name.encode(&mut self.tmp);
    }
}

impl Drop for Exports<'_> {
    fn drop(&mut self) {
        self.dst
            .finish_section(0x03, mem::replace(&mut self.tmp, Vec::new()));
    }
}

/// Writer for the list of imports in an import subsection.
pub struct Implements<'a> {
    dst: &'a mut Writer,
    tmp: Vec<u8>,
}

impl Implements<'_> {
    /// Adds a new import in this type section.
    pub fn add(&mut self, core_func: u32, adapter_func: u32) {
        core_func.encode(&mut self.tmp);
        adapter_func.encode(&mut self.tmp);
    }
}

impl Drop for Implements<'_> {
    fn drop(&mut self) {
        self.dst
            .finish_section(0x04, mem::replace(&mut self.tmp, Vec::new()));
    }
}

trait Encode {
    fn encode(&self, e: &mut Vec<u8>);
}

impl Encode for [u8] {
    fn encode(&self, e: &mut Vec<u8>) {
        self.len().encode(e);
        e.extend_from_slice(self);
    }
}

impl Encode for str {
    fn encode(&self, e: &mut Vec<u8>) {
        self.as_bytes().encode(e);
    }
}

impl Encode for usize {
    fn encode(&self, e: &mut Vec<u8>) {
        assert!(*self <= u32::max_value() as usize);
        (*self as u32).encode(e)
    }
}

impl Encode for u32 {
    fn encode(&self, e: &mut Vec<u8>) {
        leb128::write::unsigned(e, (*self).into()).unwrap();
    }
}

(module

  (func $take_i32 (param i32))
  (func $take_i64 (param i64))
  (func $return_i32 (result i32) i32.const 0)
  (func $return_i64 (result i64) i64.const 0)

  (@interface func $i32-to-s8 (result s8)
    call-core $return_i32
    i32-to-s8)
  (@interface func $i32-to-s8x (result s8)
    call-core $return_i32
    i32-to-s8x)
  (@interface func $i32-to-u8 (result u8)
    call-core $return_i32
    i32-to-u8)
  (@interface func $i32-to-s16 (result s16)
    call-core $return_i32
    i32-to-s16)
  (@interface func $i32-to-s16x (result s16)
    call-core $return_i32
    i32-to-s16x)
  (@interface func $i32-to-u16 (result u16)
    call-core $return_i32
    i32-to-u16)
  (@interface func $i32-to-s32 (result s32)
    call-core $return_i32
    i32-to-s32)
  (@interface func $i32-to-u32 (result u32)
    call-core $return_i32
    i32-to-u32)
  (@interface func $i32-to-s64 (result s64)
    call-core $return_i32
    i32-to-s64)
  (@interface func $i32-to-u64 (result u64)
    call-core $return_i32
    i32-to-u64)

  (@interface func $i64-to-s8 (result s8)
    call-core $return_i64
    i64-to-s8)
  (@interface func $i64-to-s8x (result s8)
    call-core $return_i64
    i64-to-s8x)
  (@interface func $i64-to-u8 (result u8)
    call-core $return_i64
    i64-to-u8)
  (@interface func $i64-to-s16 (result s16)
    call-core $return_i64
    i64-to-s16)
  (@interface func $i64-to-s16x (result s16)
    call-core $return_i64
    i64-to-s16x)
  (@interface func $i64-to-u16 (result u16)
    call-core $return_i64
    i64-to-u16)
  (@interface func $i64-to-s32 (result s32)
    call-core $return_i64
    i64-to-s32)
  (@interface func $i64-to-s32x (result s32)
    call-core $return_i64
    i64-to-s32x)
  (@interface func $i64-to-u32 (result u32)
    call-core $return_i64
    i64-to-u32)
  (@interface func $i64-to-s64 (result s64)
    call-core $return_i64
    i64-to-s64)
  (@interface func $i64-to-u64 (result u64)
    call-core $return_i64
    i64-to-u64)

  (@interface func (param s8)
    arg.get 0
    s8-to-i32
    call-core $take_i32)
  (@interface func (param u8)
    arg.get 0
    u8-to-i32
    call-core $take_i32)
  (@interface func (param s16)
    arg.get 0
    s16-to-i32
    call-core $take_i32)
  (@interface func (param u16)
    arg.get 0
    u16-to-i32
    call-core $take_i32)
  (@interface func (param s32)
    arg.get 0
    s32-to-i32
    call-core $take_i32)
  (@interface func (param u32)
    arg.get 0
    u32-to-i32
    call-core $take_i32)
  (@interface func (param s64)
    arg.get 0
    s64-to-i32
    call-core $take_i32)
  (@interface func (param s64)
    arg.get 0
    s64-to-i32x
    call-core $take_i32)
  (@interface func (param u64)
    arg.get 0
    u64-to-i32
    call-core $take_i32)
  (@interface func (param u64)
    arg.get 0
    u64-to-i32x
    call-core $take_i32)

  (@interface func (param s8)
    arg.get 0
    s8-to-i64
    call-core $take_i64)
  (@interface func (param u8)
    arg.get 0
    u8-to-i64
    call-core $take_i64)
  (@interface func (param s16)
    arg.get 0
    s16-to-i64
    call-core $take_i64)
  (@interface func (param u16)
    arg.get 0
    u16-to-i64
    call-core $take_i64)
  (@interface func (param s32)
    arg.get 0
    s32-to-i64
    call-core $take_i64)
  (@interface func (param u32)
    arg.get 0
    u32-to-i64
    call-core $take_i64)
  (@interface func (param s64)
    arg.get 0
    s64-to-i64
    call-core $take_i64)
  (@interface func (param u64)
    arg.get 0
    u64-to-i64
    call-core $take_i64)
)

(; CHECK-ALL:
(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i64)))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (result i64)))
  (func $take_i32 (type 0) (param i32))
  (func $take_i64 (type 1) (param i64))
  (func $return_i32 (type 2) (result i32)
    i32.const 0)
  (func $return_i64 (type 3) (result i64)
    i64.const 0)
  (@interface type (;0;) (func (result s8)))
  (@interface type (;1;) (func (result u8)))
  (@interface type (;2;) (func (result s16)))
  (@interface type (;3;) (func (result u16)))
  (@interface type (;4;) (func (result s32)))
  (@interface type (;5;) (func (result u32)))
  (@interface type (;6;) (func (result s64)))
  (@interface type (;7;) (func (result u64)))
  (@interface type (;8;) (func (param s8)))
  (@interface type (;9;) (func (param u8)))
  (@interface type (;10;) (func (param s16)))
  (@interface type (;11;) (func (param u16)))
  (@interface type (;12;) (func (param s32)))
  (@interface type (;13;) (func (param u32)))
  (@interface type (;14;) (func (param s64)))
  (@interface type (;15;) (func (param u64)))
  (@interface func (;0;) (type 0)
    call-core $return_i32
    i32-to-s8)
  (@interface func (;1;) (type 0)
    call-core $return_i32
    i32-to-s8x)
  (@interface func (;2;) (type 1)
    call-core $return_i32
    i32-to-u8)
  (@interface func (;3;) (type 2)
    call-core $return_i32
    i32-to-s16)
  (@interface func (;4;) (type 2)
    call-core $return_i32
    i32-to-s16x)
  (@interface func (;5;) (type 3)
    call-core $return_i32
    i32-to-u16)
  (@interface func (;6;) (type 4)
    call-core $return_i32
    i32-to-s32)
  (@interface func (;7;) (type 5)
    call-core $return_i32
    i32-to-u32)
  (@interface func (;8;) (type 6)
    call-core $return_i32
    i32-to-s64)
  (@interface func (;9;) (type 7)
    call-core $return_i32
    i32-to-u64)
  (@interface func (;10;) (type 0)
    call-core $return_i64
    i64-to-s8)
  (@interface func (;11;) (type 0)
    call-core $return_i64
    i64-to-s8x)
  (@interface func (;12;) (type 1)
    call-core $return_i64
    i64-to-u8)
  (@interface func (;13;) (type 2)
    call-core $return_i64
    i64-to-s16)
  (@interface func (;14;) (type 2)
    call-core $return_i64
    i64-to-s16x)
  (@interface func (;15;) (type 3)
    call-core $return_i64
    i64-to-u16)
  (@interface func (;16;) (type 4)
    call-core $return_i64
    i64-to-s32)
  (@interface func (;17;) (type 4)
    call-core $return_i64
    i64-to-s32x)
  (@interface func (;18;) (type 5)
    call-core $return_i64
    i64-to-u32)
  (@interface func (;19;) (type 6)
    call-core $return_i64
    i64-to-s64)
  (@interface func (;20;) (type 7)
    call-core $return_i64
    i64-to-u64)
  (@interface func (;21;) (type 8)
    arg.get 0
    s8-to-i32
    call-core $take_i32)
  (@interface func (;22;) (type 9)
    arg.get 0
    u8-to-i32
    call-core $take_i32)
  (@interface func (;23;) (type 10)
    arg.get 0
    s16-to-i32
    call-core $take_i32)
  (@interface func (;24;) (type 11)
    arg.get 0
    u16-to-i32
    call-core $take_i32)
  (@interface func (;25;) (type 12)
    arg.get 0
    s32-to-i32
    call-core $take_i32)
  (@interface func (;26;) (type 13)
    arg.get 0
    u32-to-i32
    call-core $take_i32)
  (@interface func (;27;) (type 14)
    arg.get 0
    s64-to-i32
    call-core $take_i32)
  (@interface func (;28;) (type 14)
    arg.get 0
    s64-to-i32x
    call-core $take_i32)
  (@interface func (;29;) (type 15)
    arg.get 0
    u64-to-i32
    call-core $take_i32)
  (@interface func (;30;) (type 15)
    arg.get 0
    u64-to-i32x
    call-core $take_i32)
  (@interface func (;31;) (type 8)
    arg.get 0
    s8-to-i64
    call-core $take_i64)
  (@interface func (;32;) (type 9)
    arg.get 0
    u8-to-i64
    call-core $take_i64)
  (@interface func (;33;) (type 10)
    arg.get 0
    s16-to-i64
    call-core $take_i64)
  (@interface func (;34;) (type 11)
    arg.get 0
    u16-to-i64
    call-core $take_i64)
  (@interface func (;35;) (type 12)
    arg.get 0
    s32-to-i64
    call-core $take_i64)
  (@interface func (;36;) (type 13)
    arg.get 0
    u32-to-i64
    call-core $take_i64)
  (@interface func (;37;) (type 14)
    arg.get 0
    s64-to-i64
    call-core $take_i64)
  (@interface func (;38;) (type 15)
    arg.get 0
    u64-to-i64
    call-core $take_i64))
;)

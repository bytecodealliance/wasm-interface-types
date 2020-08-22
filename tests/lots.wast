(module
  (import "" "" (func $foo))
  (import "" "" (func $import_return_i32 (result i32)))
  (import "" "" (func $import_return_f32 (result f32)))
  (import "" "" (func $import_return_i64 (result i64)))
  (import "" "" (func $import_return_f64 (result f64)))
  (import "" "" (func $import_take_i32 (param i32)))
  (import "" "" (func $import_take_f32 (param f32)))
  (import "" "" (func $import_take_i64 (param i64)))
  (import "" "" (func $import_take_f64 (param f64)))

  (@interface func)

  (func $return_i32 (result i32) i32.const 0)
  (func $return_f32 (result f32) f32.const 0)
  (func $return_i64 (result i64) i64.const 0)
  (func $return_f64 (result f64) f64.const 0)

  (func $take_i32 (param i32))
  (func $take_f32 (param f32))
  (func $take_i64 (param i64))
  (func $take_f64 (param f64))

  (@interface func (result s32)
    call-core $return_i32
    i32-to-s32)
  (@interface func (result s32) (result s32)
    call-core $return_i32
    i32-to-s32
    call-core $return_i32
    i32-to-s32)

  (@interface func (param s32) (result s32)
    arg.get 0)
  (@interface func (param s32) (result s32) (result s32)
    arg.get 0
    call-core $return_i32
    i32-to-s32)

  (@interface func $return_i32 (result i32)
    call-core $return_i32)
  (@interface func $return_i64 (result i64)
    call-core $return_i64)
  (@interface func $return_f32 (result f32)
    call-core $return_f32)
  (@interface func $return_f64 (result f64)
    call-core $return_f64)

  (@interface func $take_i32 (param i32)
    arg.get 0
    call-core $take_i32)
  (@interface func $take_i64 (param i64)
    arg.get 0
    call-core $take_i64)
  (@interface func $take_f32 (param f32)
    arg.get 0
    call-core $take_f32)
  (@interface func $take_f64 (param f64)
    arg.get 0
    call-core $take_f64)

  (@interface func $adapter)
  (@interface implement (func $foo) (func $adapter))

  (@interface implement (func $import_return_i32) (func $return_i32))
  (@interface implement (func $import_return_i64) (func $return_i64))
  (@interface implement (func $import_return_f32) (func $return_f32))
  (@interface implement (func $import_return_f64) (func $return_f64))

  (@interface implement (func $import_take_i32) (func $take_i32))
  (@interface implement (func $import_take_i64) (func $take_i64))
  (@interface implement (func $import_take_f32) (func $take_f32))
  (@interface implement (func $import_take_f64) (func $take_f64))
)

(; CHECK-ALL:
(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (result f32)))
  (type (;3;) (func (result i64)))
  (type (;4;) (func (result f64)))
  (type (;5;) (func (param i32)))
  (type (;6;) (func (param f32)))
  (type (;7;) (func (param i64)))
  (type (;8;) (func (param f64)))
  (import "" "" (func $foo (type 0)))
  (import "" "" (func $import_return_i32 (type 1)))
  (import "" "" (func $import_return_f32 (type 2)))
  (import "" "" (func $import_return_i64 (type 3)))
  (import "" "" (func $import_return_f64 (type 4)))
  (import "" "" (func $import_take_i32 (type 5)))
  (import "" "" (func $import_take_f32 (type 6)))
  (import "" "" (func $import_take_i64 (type 7)))
  (import "" "" (func $import_take_f64 (type 8)))
  (func $return_i32 (type 1) (result i32)
    i32.const 0)
  (func $return_f32 (type 2) (result f32)
    f32.const 0x0p+0 (;=0;))
  (func $return_i64 (type 3) (result i64)
    i64.const 0)
  (func $return_f64 (type 4) (result f64)
    f64.const 0x0p+0 (;=0;))
  (func $take_i32 (type 5) (param i32))
  (func $take_f32 (type 6) (param f32))
  (func $take_i64 (type 7) (param i64))
  (func $take_f64 (type 8) (param f64))
  (@interface type (;0;) (func))
  (@interface type (;1;) (func (result s32)))
  (@interface type (;2;) (func (result s32) (result s32)))
  (@interface type (;3;) (func (param s32) (result s32)))
  (@interface type (;4;) (func (param s32) (result s32) (result s32)))
  (@interface type (;5;) (func (result i32)))
  (@interface type (;6;) (func (result i64)))
  (@interface type (;7;) (func (result f32)))
  (@interface type (;8;) (func (result f64)))
  (@interface type (;9;) (func (param i32)))
  (@interface type (;10;) (func (param i64)))
  (@interface type (;11;) (func (param f32)))
  (@interface type (;12;) (func (param f64)))
  (@interface func (;0;) (type 0))
  (@interface func (;1;) (type 1)
    call-core $return_i32
    i32-to-s32)
  (@interface func (;2;) (type 2)
    call-core $return_i32
    i32-to-s32
    call-core $return_i32
    i32-to-s32)
  (@interface func (;3;) (type 3)
    arg.get 0)
  (@interface func (;4;) (type 4)
    arg.get 0
    call-core $return_i32
    i32-to-s32)
  (@interface func (;5;) (type 5)
    call-core $return_i32)
  (@interface func (;6;) (type 6)
    call-core $return_i64)
  (@interface func (;7;) (type 7)
    call-core $return_f32)
  (@interface func (;8;) (type 8)
    call-core $return_f64)
  (@interface func (;9;) (type 9)
    arg.get 0
    call-core $take_i32)
  (@interface func (;10;) (type 10)
    arg.get 0
    call-core $take_i64)
  (@interface func (;11;) (type 11)
    arg.get 0
    call-core $take_f32)
  (@interface func (;12;) (type 12)
    arg.get 0
    call-core $take_f64)
  (@interface func (;13;) (type 0))
  (@interface implement (func $foo) (func 13))
  (@interface implement (func $import_return_i32) (func 5))
  (@interface implement (func $import_return_i64) (func 6))
  (@interface implement (func $import_return_f32) (func 7))
  (@interface implement (func $import_return_f64) (func 8))
  (@interface implement (func $import_take_i32) (func 9))
  (@interface implement (func $import_take_i64) (func 10))
  (@interface implement (func $import_take_f32) (func 11))
  (@interface implement (func $import_take_f64) (func 12)))
;)

(module
  (func $foo (param i32 f32))
  (@interface func $adapter_foo (export "foo") (param s32) (param f32)
    arg.get 0
    s32-to-i32
    arg.get 1
    call-core $foo)
  (@interface func (export "bar") (param s32) (param f32)
    arg.get 0
    arg.get 1
    call-adapter $adapter_foo))

(; CHECK-ALL:
(module
  (type (;0;) (func (param i32 f32)))
  (func $foo (type 0) (param i32 f32))
  (@interface type (;0;) (func (param s32) (param f32)))
  (@interface func (;0;) (type 0)
    arg.get 0
    s32-to-i32
    arg.get 1
    call-core $foo)
  (@interface func (;1;) (type 0)
    arg.get 0
    arg.get 1
    call-adapter 0)
  (@interface export "foo" (func 0))
  (@interface export "bar" (func 1)))
;)

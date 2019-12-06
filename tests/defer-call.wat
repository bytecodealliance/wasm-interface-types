(module
  (memory 1)
  (func $free (param i32))
  (func $greeting (result i32 i32)
    i32.const 0
    i32.const 0)
  (@interface func (export "greeting") (result string)
    call-core $greeting
    defer-call-core $free
    memory-to-string)
)

(; CHECK-ALL:
(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (result i32 i32)))
  (func $free (type 0) (param i32))
  (func $greeting (type 1) (result i32 i32)
    i32.const 0
    i32.const 0)
  (memory (;0;) 1)
  (@interface type (;0;) (func (result string)))
  (@interface func (;0;) (type 0)
    call-core $greeting
    defer-call-core $free
    memory-to-string)
  (@interface export "greeting" (func 0)))
;)

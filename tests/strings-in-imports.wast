(module
  (func (import "" "log_") (param i32 i32))
  (memory 1)

  (@interface func $log (import "" "log") (param $arg string))
  (@interface implement (import "" "log_") (param $ptr i32) (param $len i32)
    arg.get $ptr
    arg.get $len
    memory-to-string
    call-adapter 0
  )
)

(; CHECK-ALL:
(module
  (type (;0;) (func (param i32 i32)))
  (import "" "log_" (func (;0;) (type 0)))
  (memory (;0;) 1)
  (@interface type (;0;) (func (param string)))
  (@interface type (;1;) (func (param i32) (param i32)))
  (@interface import "" "log" (func (;0;) (type 0)))
  (@interface func (;1;) (type 1)
    arg.get 0
    arg.get 1
    memory-to-string
    call-adapter 0)
  (@interface implement (func 0) (func 1)))
;)

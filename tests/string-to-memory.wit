(module
  (memory $mem 1)
  (func $malloc (param i32) (result i32) i32.const 0)
  (func $free (param i32))
  (func $log (param i32 i32))
  (@interface func (export "log") (param $str string)
    arg.get $str
    string-to-memory $malloc $mem
    call-core $log
  )


  ;; alternative syntaxes
  (@interface func (param $str string)
    arg.get $str
    string-to-memory $malloc ;; no mem
    call-core $log
  )
  (@interface func (param $str string)
    arg.get $str
    string-to-memory $malloc 0
    call-core $log
  )
  (@interface func (param $str string)
    arg.get $str
    string-to-memory 0 $mem
    call-core $log
  )
  (@interface func (param $str string)
    arg.get $str
    string-to-memory 0 0
    call-core $log
  )
)

(; CHECK-ALL:
(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32 i32)))
  (func $malloc (type 0) (param i32) (result i32)
    i32.const 0)
  (func $free (type 1) (param i32))
  (func $log (type 2) (param i32 i32))
  (memory (;0;) 1)
  (@interface type (;0;) (func (param string)))
  (@interface func (;0;) (type 0)
    arg.get 0
    string-to-memory $malloc
    call-core $log)
  (@interface func (;1;) (type 0)
    arg.get 0
    string-to-memory $malloc
    call-core $log)
  (@interface func (;2;) (type 0)
    arg.get 0
    string-to-memory $malloc
    call-core $log)
  (@interface func (;3;) (type 0)
    arg.get 0
    string-to-memory $malloc
    call-core $log)
  (@interface func (;4;) (type 0)
    arg.get 0
    string-to-memory $malloc
    call-core $log)
  (@interface export "log" (func 0)))
;)

;; validate-fail

(module
  (memory $mem 1)
  (func $malloc)
  (func $log (param i32 i32))
  (@interface func (export "log") (param $str string)
    arg.get $str
    string-to-memory $malloc $mem
    call-core $log
  )
)

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter func 0
    1: malloc function 0 does not have correct signature
;)

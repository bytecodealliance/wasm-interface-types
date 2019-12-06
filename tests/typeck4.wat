;; validate-fail

(module
  (func (param i32))
  (@interface func (param s64) (result s32)
    arg.get 0
    call-core 0))

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter func 0
    1: expected I32 on type stack, found S64
;)

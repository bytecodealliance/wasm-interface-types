;; validate-fail
(module
  (func $foo (param i32))
  (@interface func (param s32)
    arg.get 0
    call-core $foo)
)

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter func 0
    1: expected I32 on type stack, found S32
;)

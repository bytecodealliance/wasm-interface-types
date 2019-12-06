;; validate-fail

(module
  (@interface func (param s64) (result s32)
    arg.get 0))

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter func 0
    1: expected S32 on type stack, found S64
;)

;; validate-fail

(module
  (@interface import "" "" (func (type 1))))

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter import 0
    1: adapter type index too large: 1
;)

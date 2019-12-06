;; validate-fail

(module
  (@interface func (type 0)))

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter func 0
    1: adapter type index too large: 0
;)

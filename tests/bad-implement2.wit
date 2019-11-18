;; validate-fail

(module
  (@interface implement (func 0) (func 0))
  (@interface func)
)

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter implement 0
    1: func index too large: 0
;)

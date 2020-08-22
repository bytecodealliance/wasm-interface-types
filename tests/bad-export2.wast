;; validate-fail

(module
  (@interface export "x" (func 0))
  (@interface export "x" (func 0))

  (@interface func)
  )

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter export 1
    1: found duplicate export `x`
;)

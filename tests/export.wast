;; no-validate

(module
  (@interface export "x" (func 0))
)

(; CHECK-ALL:
(module
  (@interface export "x" (func 0)))
;)

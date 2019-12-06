;; validate-fail

(module
  (import "" "" (func (result i32)))
  (@interface implement (import "" ""))
)

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter implement 0
    1: core function 0 has a different type signature than adapter function 0
;)

;; validate-fail

(module
  (import "" "" (func))
  (@interface implement (import "" "") (result s32)))

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter func 0
    1: expected S32 on type stack, found nothing
;)

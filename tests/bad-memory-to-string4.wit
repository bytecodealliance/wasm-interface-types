;; validate-fail

(module
  (memory 1)
  (@interface func (export "greeting") (result string)
    memory-to-string
  )
)

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter func 0
    1: expected I32 on type stack, found nothing
;)

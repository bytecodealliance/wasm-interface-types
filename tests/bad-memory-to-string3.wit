;; validate-fail

(module
  (func $greeting (result i32 i32)
    i32.const 0    ;; offset of string in memory
    i32.const 11   ;; length
  )
  (@interface func (export "greeting") (result string)
    call-core $greeting
    memory-to-string
  )
)

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to validate adapter func 0
    1: memory index out of bounds: 0
;)

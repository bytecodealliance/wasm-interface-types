;; parse-fail

(module
  (func $greeting (result i32 i32)
    i32.const 0    ;; offset of string in memory
    i32.const 11   ;; length
  )
  (@interface func (export "greeting") (result string)
    call-core $greeting
    memory-to-string $mem
  )
)

(; CHECK-ALL:
failed to find memory named `$mem`
     --> tests/bad-memory-to-string.wat:10:22
      |
   10 |     memory-to-string $mem
      |                      ^
;)

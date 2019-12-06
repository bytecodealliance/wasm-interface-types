;; parse-fail

(module
  (@interface export "x" (func $foo)))

(; CHECK-ALL:
failed to find adapter func named `$foo`
     --> tests/unresolved2.wat:4:32
      |
    4 |   (@interface export "x" (func $foo)))
      |                                ^
;)

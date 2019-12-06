;; parse-fail

(module
  (@interface type foo))

(; CHECK-ALL:
expected `(`
     --> tests/types-invalid.wat:4:20
      |
    4 |   (@interface type foo))
      |                    ^
;)

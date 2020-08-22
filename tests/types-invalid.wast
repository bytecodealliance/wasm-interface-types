;; parse-fail

(module
  (@interface type foo))

(; CHECK-ALL:
expected `(`
     --> tests/types-invalid.wast:4:20
      |
    4 |   (@interface type foo))
      |                    ^
;)

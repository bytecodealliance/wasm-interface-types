;; parse-fail

x

(; CHECK-ALL:
expected `(`
     --> tests/not-interface.wat:3:1
      |
    3 | x
      | ^
;)

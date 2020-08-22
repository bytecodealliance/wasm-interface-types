;; parse-fail

x

(; CHECK-ALL:
expected `(`
     --> tests/not-interface.wast:3:1
      |
    3 | x
      | ^
;)

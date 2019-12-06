;; parse-fail

(module
  (@interface type (result s8) (param u8)))

(; CHECK-ALL:
expected keyword `func`
     --> tests/types-wrong-order.wat:4:21
      |
    4 |   (@interface type (result s8) (param u8)))
      |                     ^
;)

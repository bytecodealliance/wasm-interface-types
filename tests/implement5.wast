;; parse-fail

(module
  (func $foo)
  (@interface implement (func 1) (func $foo))
)

(; CHECK-ALL:
failed to find adapter func named `$foo`
     --> tests/implement5.wast:5:40
      |
    5 |   (@interface implement (func 1) (func $foo))
      |                                        ^
;)

;; parse-fail

(module
  (@interface implement (func $foo) (func 1))
  (@interface func $foo)
)

(; CHECK-ALL:
failed to find func named `$foo`
     --> tests/implement4.wast:4:31
      |
    4 |   (@interface implement (func $foo) (func 1))
      |                               ^
;)

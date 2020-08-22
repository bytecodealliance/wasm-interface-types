;; parse-fail

(module
  (import "" "" (global i32))
  (@interface implement (import "" "") (func 1)))

(; CHECK-ALL:
import of `` from `` not found in core module
     --> tests/implement2.wast:5:15
      |
    5 |   (@interface implement (import "" "") (func 1)))
      |               ^
;)

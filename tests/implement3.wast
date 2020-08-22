;; parse-fail

(module
  (import "" "" (func))
  (import "" "" (func))
  (@interface implement (import "" "") (func 1)))

(; CHECK-ALL:
import of `` from `` is ambiguous since it's listed twice in the core module
     --> tests/implement3.wast:6:15
      |
    6 |   (@interface implement (import "" "") (func 1)))
      |               ^
;)

;; parse-fail

(module
  (@interface implement (import "" "") (func 1)))

(; CHECK-ALL:
import of `` from `` not found in core module
     --> tests/implement1.wast:4:15
      |
    4 |   (@interface implement (import "" "") (func 1)))
      |               ^
;)

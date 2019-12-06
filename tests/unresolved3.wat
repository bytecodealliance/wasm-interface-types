;; parse-fail

(module
  (@interface import "x" "y" (func (type $foo))))

(; CHECK-ALL:
failed to find type named `$foo`
     --> tests/unresolved3.wat:4:42
      |
    4 |   (@interface import "x" "y" (func (type $foo))))
      |                                          ^
;)

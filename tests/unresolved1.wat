;; parse-fail

(module
  (@interface func (type $foo)))

(; CHECK-ALL:
failed to find type named `$foo`
     --> tests/unresolved1.wat:4:26
      |
    4 |   (@interface func (type $foo)))
      |                          ^
;)

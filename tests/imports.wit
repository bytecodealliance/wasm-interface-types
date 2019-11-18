(module
  (@interface type $foo (func (param s32)))
  (@interface import "foo" "bar" (func $foo))
  (@interface import "foo" "bar" (func $bar (type $foo)))
  (@interface import "" "" (func (type 0)))
)

(; CHECK-ALL:
(module
  (@interface type (;0;) (func (param s32)))
  (@interface type (;1;) (func))
  (@interface import "foo" "bar" (func (;0;) (type 1)))
  (@interface import "foo" "bar" (func (;1;) (type 0)))
  (@interface import "" "" (func (;2;) (type 0))))
;)

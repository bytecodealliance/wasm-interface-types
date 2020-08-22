(func)
(@interface type (func))

(; CHECK-ALL:
(module
  (type (;0;) (func))
  (func (;0;) (type 0))
  (@interface type (;0;) (func)))
;)

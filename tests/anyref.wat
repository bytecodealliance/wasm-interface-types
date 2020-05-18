(module
  (@interface func (param anyref))
  (@interface func $get_anyref (import "" "") (result anyref))

  (func $take_anyref (param anyref))

  (@interface func
    call-adapter $get_anyref
    call-core $take_anyref)
)

(; CHECK-ALL:
(module
  (type (;0;) (func (param externref)))
  (func $take_anyref (type 0) (param externref))
  (@interface type (;0;) (func (param anyref)))
  (@interface type (;1;) (func (result anyref)))
  (@interface type (;2;) (func))
  (@interface import "" "" (func (;0;) (type 1)))
  (@interface func (;1;) (type 0))
  (@interface func (;2;) (type 2)
    call-adapter 0
    call-core $take_anyref))
;)

(module
  (@interface func (param externref))
  (@interface func $get_externref (import "" "") (result externref))

  (func $take_externref (param externref))

  (@interface func
    call-adapter $get_externref
    call-core $take_externref)
)

(; CHECK-ALL:
(module
  (type (;0;) (func (param externref)))
  (func $take_externref (type 0) (param externref))
  (@interface type (;0;) (func (param externref)))
  (@interface type (;1;) (func (result externref)))
  (@interface type (;2;) (func))
  (@interface import "" "" (func (;0;) (type 1)))
  (@interface func (;1;) (type 0))
  (@interface func (;2;) (type 2)
    call-adapter 0
    call-core $take_externref))
;)

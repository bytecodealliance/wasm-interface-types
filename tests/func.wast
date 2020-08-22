;; no-validate

(module
  (@interface func)
  (@interface func (param $foo string)
    arg.get $foo
  )
)

(; CHECK-ALL:
(module
  (@interface type (;0;) (func))
  (@interface type (;1;) (func (param string)))
  (@interface func (;0;) (type 0))
  (@interface func (;1;) (type 1)
    arg.get 0))
;)

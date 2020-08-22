;; no-validate

(module
  (import "a" "b" (func $core))
  (import "b" "c" (func))
  (import "b" "c" (func $bar))

  (@interface implement (import "a" "b") (func 100))
  (@interface implement (func $bar) (func 100))
  (@interface implement (func 0) (func 100))

  (@interface implement (func $core))
  (@interface implement (func 0) (param s32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func))
  (import "a" "b" (func $core (type 0)))
  (import "b" "c" (func (;1;) (type 0)))
  (import "b" "c" (func $bar (type 0)))
  (@interface type (;0;) (func))
  (@interface type (;1;) (func (param s32)))
  (@interface func (;0;) (type 0))
  (@interface func (;1;) (type 1))
  (@interface implement (func $core) (func 100))
  (@interface implement (func $bar) (func 100))
  (@interface implement (func $core) (func 100))
  (@interface implement (func $core) (func 0))
  (@interface implement (func $core) (func 1)))
;)

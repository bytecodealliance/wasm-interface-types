;; validate-fail

(module binary
  "\00asm"
  "\01\00\00\00"

  "\00"                     ;; custom section id
  "\15"                     ;; size of section
  "\14"                     ;; size of section name
  "wasm-interface-types"    ;; name of section

  "\00"                     ;; custom section id
  "\15"                     ;; size of section
  "\14"                     ;; size of section name
  "wasm-interface-types"    ;; name of section
)

(; CHECK-ALL:
  found two `wasm-interface-types` custom sections
;)

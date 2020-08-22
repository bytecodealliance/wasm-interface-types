;; validate-fail

(module binary
  "\00asm"
  "\01\00\00\00"

  "\00"                     ;; custom section id
  "\1b"                     ;; size of section
  "\14"                     ;; size of section name
  "wasm-interface-types"    ;; name of section (20 bytes)
  "\05"                     ;; schema version len
  "0.1.0"                   ;; schema version (5 bytes)


  "\00"                     ;; custom section id
  "\15"                     ;; size of section
  "\14"                     ;; size of section name
  "wasm-interface-types"    ;; name of section
)

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    found two `wasm-interface-types` custom sections
;)

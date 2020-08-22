;; validate-fail

(module binary
  "\00asm"
  "\01\00\00\00"

  "\00"                     ;; custom section id
  "\1b"                     ;; size of section
  "\14"                     ;; size of section name
  "wasm-interface-types"    ;; name of section (20 bytes)
  "\05"                     ;; schema version len
  "0.0.0"                   ;; schema version (5 bytes)
)

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to parse interface types header
    1: failed to parse at byte 0: schema version `0.0.0` doesn't match `0.1.0`
;)

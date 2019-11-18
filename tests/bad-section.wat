;; validate-fail

(module binary
  "\00asm"
  "\01\00\00\00"

  "\00"                     ;; custom section id
  "\1d"                     ;; size of section
  "\14"                     ;; size of section name
  "wasm-interface-types"    ;; name of section (20 bytes)
  "\05"                     ;; schema version len
  "0.1.0"
  "\7f"                     ;; section number
  "\00"                     ;; section size
)

(; CHECK-ALL:
failed to validate interface types section

Caused by:
    0: failed to read section header
    1: failed to parse at byte 6: invalid section id: 127
;)

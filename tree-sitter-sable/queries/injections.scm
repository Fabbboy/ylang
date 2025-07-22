; Future: String interpolation or embedded languages
; Example for when you add string literals with embedded expressions
; (string_literal
;   (escape_sequence) @injection.content
;   (#set! injection.language "c"))

; Future: Documentation comments with markdown
; ((comment) @injection.content
;  (#match? @injection.content "^///.*")
;  (#set! injection.language "markdown"))

; Keywords
"func" @keyword
"var" @keyword

; Literals
(integer_literal) @number
(float_literal) @number

; Identifiers
(identifier) @variable
(type (identifier) @type)

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"=" @operator

; Delimiters
[":" ";" ","] @punctuation.delimiter
["(" ")" "{" "}"] @punctuation.bracket

; Comments
(comment) @comment

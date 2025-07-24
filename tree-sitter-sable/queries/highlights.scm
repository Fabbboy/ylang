; Keywords
"func" @keyword.function
"var" @keyword

; Function definitions
(function_declaration
  (identifier) @function)

; Types
(type
  (identifier) @type)

; Pointer suffixes
(pointer_suffix) @type.qualifier

; Variables (parameters are more specific than general variables)
(parameter
  (identifier) @parameter)

(variable_declaration
  (identifier) @variable)

; Literals
(integer_literal) @number
(float_literal) @number.float

(semi) @operators
"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator

; Punctuation
(lparent) @punctuation.bracket
(rparent) @punctuation.bracket
(lbrace) @punctuation.bracket
(rbrace) @punctuation.bracket
"," @punctuation.delimiter
(colon) @punctuation.delimiter

; Comments
(comment) @comment

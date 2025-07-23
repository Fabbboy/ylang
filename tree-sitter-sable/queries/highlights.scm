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

; Operators
"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
";" @punctuation.delimiter
"," @punctuation.delimiter
":" @punctuation.delimiter

; Comments
(comment) @comment

; Keywords
(func_kw) @keyword.function
(var_kw) @keyword

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
equal: ($) => "=",
plus: ($) => "+",
minus: ($) => "-",
star: ($) => "*",
slash: ($) => "/",

; Punctuation
(lparent) @punctuation.bracket
(rparent) @punctuation.bracket
(lbrace) @punctuation.bracket
(rbrace) @punctuation.bracket
(comma) @punctuation.delimiter
(colon) @punctuation.delimiter

; Comments
(comment) @comment

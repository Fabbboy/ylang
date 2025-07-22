; Keywords
"func" @keyword.function
"var" @keyword

; Function definitions
(function_declaration
  (identifier) @function)

; Types - built-in types (more specific patterns first)
(type 
  (identifier) @type.builtin
  (#match? @type.builtin "^(i32|i64|i8|i16|u32|u64|u8|u16|f32|f64|bool|void|char|str)$"))

; Types - all other types (user-defined, custom types)
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

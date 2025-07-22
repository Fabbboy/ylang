; Fold blocks
(block) @fold

; Fold function declarations
(function_declaration
  body: (block) @fold)

; Fold parameter lists (if they span multiple lines)
(parameter_list) @fold

; Fold comments (if you want multi-line comment folding)
(comment) @fold

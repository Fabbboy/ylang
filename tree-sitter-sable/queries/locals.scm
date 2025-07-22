; Scopes
(block) @local.scope
(function_declaration) @local.scope

; Definitions
(function_declaration
  (identifier) @local.definition)

(variable_declaration
  (identifier) @local.definition)

(parameter
  (identifier) @local.definition)

; References
(identifier) @local.reference

lexer grammar SableLexer;

FUNC        : 'func';
VAR         : 'var';
COLON       : ':';
SEMI        : ';';
COMMA       : ',';
ASSIGN      : '=';
LPAREN      : '(';
RPAREN      : ')';
LBRACE      : '{';
RBRACE      : '}';
PLUS        : '+';
MINUS       : '-';
STAR        : '*';
SLASH       : '/';

IntegerLiteral
  : [0-9]+
  ;

FloatLiteral
  : [0-9]+ '.' [0-9]+
  ;

Identifier
  : [a-zA-Z_][a-zA-Z0-9_]*
  ;

WS
  : [ \t\r\n]+ -> skip
  ;

COMMENT
  : '//' ~[\r\n]* -> skip
  ;


parser grammar SableParser;

options { tokenVocab=SableLexer; }

program
  : (functionDecl)* EOF
  ;

functionDecl
  : FUNC identifier LPAREN parameterList? RPAREN COLON type blockOrSemi
  ;

parameterList
  : parameter (COMMA parameter)*
  ;

parameter
  : identifier COLON type
  ;

type
  : identifier pointerSuffix?
  ;

pointerSuffix
  : (STAR)+
  ;

blockOrSemi
  : block
  | SEMI
  ;

block
  : LBRACE statement* RBRACE
  ;

statement
  : variableDecl
  | expressionStmt
  ;

variableDecl
  : VAR identifier (COLON type)? ASSIGN expression SEMI
  ;

expressionStmt
  : expression SEMI
  ;

expression
  : assignment
  ;

assignment
  : identifier ASSIGN expression
  | additive
  ;

additive
  : multiplicative ((PLUS | MINUS) multiplicative)*
  ;

multiplicative
  : primary ((STAR | SLASH) primary)*
  ;

primary
  : literal
  | identifier
  | LPAREN expression RPAREN
  ;

literal
  : IntegerLiteral
  | FloatLiteral
  ;

identifier
  : Identifier
  ;


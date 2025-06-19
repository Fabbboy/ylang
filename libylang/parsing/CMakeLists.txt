set(LOCAL_SOURCES 
  # General
  Location.h
  Location.cc

  # Lexer
  Lexer/Token.h
  Lexer/Token.cc
  Lexer/Lexer.h 
  Lexer/Lexer.cc

  # AST
  Ast/Visitor.h 
  Ast/Ast.h

  # Parser 
  Parser/Parser.h
  Parser/Parser.cc
)

set(LOCAL_SOURCES_WITH_PATH)
foreach(file ${LOCAL_SOURCES})
  list(APPEND LOCAL_SOURCES_WITH_PATH ${CMAKE_CURRENT_SOURCE_DIR}/${file})
endforeach()

list(APPEND SOURCES ${LOCAL_SOURCES_WITH_PATH})
set(SOURCES ${SOURCES} PARENT_SCOPE)
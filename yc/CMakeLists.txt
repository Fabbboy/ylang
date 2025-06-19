set(SOURCES  
  main.cc 
)

add_executable(yc ${SOURCES})
target_link_libraries(yc ylang)
target_compile_options(yc PRIVATE -g)
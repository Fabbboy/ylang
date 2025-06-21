set(SOURCES  
  main.cc 
)

add_executable(sc ${SOURCES})
target_link_libraries(sc sable)
target_compile_options(sc PRIVATE -g)
set(SOURCES  
  main.cc 
)

add_executable(yc ${SOURCES})
target_link_libraries(yc ylang)
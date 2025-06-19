set(SOURCES 
  Range.h
  Manager.h
  Manager.cc
)

add_subdirectory(parsing)
add_subdirectory(report)

message(STATUS ${SOURCES})

add_library(ylang STATIC ${SOURCES})
target_include_directories(ylang PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(ylang PRIVATE -g)
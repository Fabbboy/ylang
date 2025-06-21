set(SOURCES )

add_subdirectory(common)
add_subdirectory(parsing)
add_subdirectory(report)

add_library(sable STATIC ${SOURCES})
target_include_directories(sable PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(sable PRIVATE -g)
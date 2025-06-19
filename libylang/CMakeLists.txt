set(SOURCES "")

add_subdirectory(parsing)

add_library(ylang STATIC ${SOURCES})
target_include_directories(ylang PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
set(LOCAL_SOURCES
  Cache.h
  Cache.cc
  Diagnostic.h
  Diagnostic.cc
  Reporter.h
  Reporter.cc
)

set(LOCAL_SOURCES_WITH_PATH)
foreach(file ${LOCAL_SOURCES})
  list(APPEND LOCAL_SOURCES_WITH_PATH ${CMAKE_CURRENT_SOURCE_DIR}/${file})
endforeach()

list(APPEND SOURCES ${LOCAL_SOURCES_WITH_PATH})
set(SOURCES ${SOURCES} PARENT_SCOPE)
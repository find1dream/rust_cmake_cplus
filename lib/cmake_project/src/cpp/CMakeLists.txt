cmake_minimum_required(VERSION 3.12)
project("Rust_call_C++" LANGUAGES CXX C)

set(TARGET cmake_project)

add_library(${TARGET}
  STATIC
    func.cc
)

target_include_directories(${TARGET}
  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
)

install (TARGETS ${TARGET} DESTINATION .)

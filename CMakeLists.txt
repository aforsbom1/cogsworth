cmake_minimum_required(VERSION 3.10)
project(cogsworth C)

set(CMAKE_C_STANDARD 99)

set(EXECUTABLE_OUTPUT_PATH ${CMAKE_SOURCE_DIR}/build)

file(GLOB_RECURSE SOURCES "src/*.c")

add_executable(cogsworth ${SOURCES})

target_link_libraries(cogsworth m)
cmake_minimum_required(VERSION 3.20)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_EXECUTABLE_SUFFIX_CXX ".elf")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)


if (NOT PLATFORM)
    message(FATAL "Missing PLATFORM")
endif()

project(memdump_testapp_${PLATFORM} CXX ASM)

set(PLATFORM_DIR "${CMAKE_CURRENT_SOURCE_DIR}/platform/${PLATFORM}")

add_executable(${PROJECT_NAME} 
    src/main.cpp
    src/file1.cpp
    src/file2.cpp
    src/file3.cpp
    src/file4.cpp
    src/file5.cpp
    ${PLATFORM_DIR}/startup.S 
    ${PLATFORM_DIR}/uart.cpp
)

target_include_directories(${PROJECT_NAME} PRIVATE include)

target_compile_options(${PROJECT_NAME} PRIVATE -O0 -Wall -Wextra -gdwarf-4)

target_link_options(${PROJECT_NAME} PRIVATE
    -T ${PLATFORM_DIR}/linker.ld
    -nostartfiles
    -nostdlib
)

cmake_minimum_required(VERSION 3.20)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_EXECUTABLE_SUFFIX_CXX ".elf")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

project(leon3_hello CXX ASM)
add_executable(${PROJECT_NAME} 
    startup.S 
    src/main.cpp
    src/file1.cpp
    src/file2.cpp
    src/file3.cpp
    src/file4.cpp
    src/file5.cpp
    src/term_parser.cpp
    src/uart.cpp
)

target_include_directories(${PROJECT_NAME} PRIVATE include)

target_compile_options(${PROJECT_NAME} PRIVATE -O2 -Wall -Wextra)

target_link_options(${PROJECT_NAME} PRIVATE
    -T ${CMAKE_CURRENT_SOURCE_DIR}/linker.ld
    -nostartfiles
    -nostdlib
)

# Print section sizes after every build
find_program(SPARC_SIZE sparc-elf-size)
if(SPARC_SIZE)
    add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${SPARC_SIZE} $<TARGET_FILE:${PROJECT_NAME}>
        VERBATIM
    )
endif()

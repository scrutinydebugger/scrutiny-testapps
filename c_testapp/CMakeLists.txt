# Copyright (c) 2021-2023 Scrutiny Debugger
# License : MIT - See LICENSE file.
# Project : Scrutiny Debugger (github.com/scrutinydebugger/scrutiny-embedded)

cmake_minimum_required(VERSION 3.14)
project(c_testapp LANGUAGES C CXX)

# The library CI redirects this to its own checkout with -DFETCHCONTENT_SOURCE_DIR_SCRUTINY_EMBEDDED=<path>
set(SCRUTINY_EMBEDDED_GIT_TAG master CACHE STRING "scrutiny-embedded branch, tag or commit to fetch")
include(FetchContent)
FetchContent_Declare(scrutiny_embedded
    GIT_REPOSITORY https://github.com/scrutinydebugger/scrutiny-embedded
    GIT_TAG ${SCRUTINY_EMBEDDED_GIT_TAG}
    GIT_SHALLOW TRUE
)
set(SCRUTINY_BUILD_CWRAPPER ON)
FetchContent_MakeAvailable(scrutiny_embedded)

add_executable(${PROJECT_NAME})

set_target_properties(${PROJECT_NAME} PROPERTIES C_STANDARD 99)

target_sources(${PROJECT_NAME} PRIVATE
    ${CMAKE_CURRENT_LIST_DIR}/src/main.c
    ${CMAKE_CURRENT_LIST_DIR}/src/file1.c
    ${CMAKE_CURRENT_LIST_DIR}/src/file2.c
    ${CMAKE_CURRENT_LIST_DIR}/src/tools.c
    ${CMAKE_CURRENT_LIST_DIR}/src/include_test.c
    ${CMAKE_CURRENT_LIST_DIR}/src/argument_parser.c
    ${CMAKE_CURRENT_LIST_DIR}/src/comm_channels/udp_bridge.c
)

if (WIN32)
    target_link_libraries(${PROJECT_NAME}
        wsock32
    )

    target_sources(${PROJECT_NAME} PRIVATE
        ${CMAKE_CURRENT_LIST_DIR}/src/comm_channels/win_serial_port_bridge.c
    )
else()
    target_sources(${PROJECT_NAME} PRIVATE
        ${CMAKE_CURRENT_LIST_DIR}/src/comm_channels/nix_serial_port_bridge.c
    )
endif ()

if (SCRUTINY_CWRAPPER_EXTRACT_CPP_CONSTANTS)
    target_compile_definitions(${PROJECT_NAME} PRIVATE SCRUTINY_CWRAPPER_EXTRACT_CPP_CONSTANTS)
endif()

target_include_directories(${PROJECT_NAME} PRIVATE
   ${CMAKE_CURRENT_LIST_DIR}/include
   ${CMAKE_CURRENT_LIST_DIR}/include/comm_channels
)

target_link_libraries(${PROJECT_NAME}
    scrutiny-cwrapper
)

if (CMAKE_C_COMPILER_ID STREQUAL "MSVC")    # MSVC doesn't support dwarf output. We still allows build of testapp.
    target_compile_options(${PROJECT_NAME} PRIVATE
        /W4 /Od /WX
    )

    target_link_options(${PROJECT_NAME} PUBLIC
        /DYNAMICBASE:NO
    )

else()
    if (CMAKE_C_COMPILER_ID STREQUAL "Clang")
        set(COMPILER_NO_PIE_OPTION "-fno-pie")
        set(LINKER_NO_PIE_OPTION "-no-pie") # clang use system linker.
    elseif (CMAKE_C_COMPILER_ID STREQUAL "GNU")
        set(COMPILER_NO_PIE_OPTION "-no-pie")
        set(LINKER_NO_PIE_OPTION "-no-pie")
    else()
        set(COMPILER_NO_PIE_OPTION "")
        set(LINKER_NO_PIE_OPTION "")
    endif()

    target_compile_options(${PROJECT_NAME} PRIVATE
        ${COMPILER_NO_PIE_OPTION}
        -gdwarf-4
        -O0
        -Wall -Wextra -Wpedantic -Werror
    )

    target_link_options(${PROJECT_NAME} PUBLIC
        ${LINKER_NO_PIE_OPTION}
        -gdwarf-4
    )

    scrutiny_postbuild(${PROJECT_NAME}
        INSTALL_SFD

        METADATA_PROJECT_NAME "Scrutiny CTestAPP"
        METADATA_AUTHOR ""
        METADATA_VERSION "1.0.0"

        ALIAS_FILES
            assets/aliases.json
    )

endif()

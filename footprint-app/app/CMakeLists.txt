cmake_minimum_required(VERSION 3.14)
project(footprint-app)
include(FetchContent)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(SCRUTINY_BUILD_CWRAPPER  OFF)
set(SCRUTINY_ENABLE_DATALOGGING   ON)
set(SCRUTINY_SUPPORT_64BITS   OFF)

FetchContent_Declare(
  scrutiny-embedded
  GIT_REPOSITORY https://github.com/scrutinydebugger/scrutiny-embedded.git
  GIT_TAG        master
)

add_compile_options(-Wall -Wextra -Werror -fstack-usage -fcallgraph-info=su -fdump-rtl-expand)

FetchContent_MakeAvailable(scrutiny-embedded)
target_compile_options(scrutiny-embedded PRIVATE -Os)
add_executable(${PROJECT_NAME} main.cpp)

target_link_libraries(${PROJECT_NAME} PRIVATE
    scrutiny-embedded
)

target_link_options(${PROJECT_NAME} PRIVATE -Wl,-Map=${PROJECT_NAME}.map)
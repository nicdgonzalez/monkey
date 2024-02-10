cmake_minimum_required(VERSION 3.20.2)

# Generates a `compile_commands.json` file in the build directory (to be used
# by the C++ Language Server to better understand the project structure).
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(
    monkey
    VERSION 0.1.0
    DESCRIPTION "Writing an interpreter for the Monkey programming language."
    HOMEPAGE_URL "https://github.com/nicdgonzalez/monkey"
    LANGUAGES CXX
)

add_executable(${PROJECT_NAME} "./src/main.cc")

set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD 20)
set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD_REQUIRED ON)
set_property(TARGET ${PROJECT_NAME} PROPERTY COMPILE_OPTIONS "-Wall;-Werror;-Wpedantic")

file(GLOB MONKEY_SOURCES "${PROJECT_SOURCE_DIR}/src/**/*.cc")
target_sources(${PROJECT_NAME} PRIVATE ${MONKEY_SOURCES})

target_include_directories(
    ${PROJECT_NAME}
    PRIVATE
    "${PROJECT_SOURCE_DIR}/include"
)

enable_testing()
add_subdirectory(tests)

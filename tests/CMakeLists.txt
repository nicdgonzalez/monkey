include(FetchContent)

FetchContent_Declare(
    googletest
    GIT_REPOSITORY "https://github.com/google/googletest.git"
    GIT_TAG v1.14.0
)

FetchContent_MakeAvailable(googletest)

# Use MONKEY_TEST for test-related executable binaries.
set(MONKEY_TEST ${CMAKE_PROJECT_NAME}_test)

file(GLOB MONKEY_TEST_SOURCES "test_*.cc")
add_executable(${MONKEY_TEST} ${MONKEY_TEST_SOURCES})

set_property(TARGET ${MONKEY_TEST} PROPERTY CXX_STANDARD 20)
set_property(TARGET ${MONKEY_TEST} PROPERTY CXX_STANDARD_REQUIRED ON)
set_property(TARGET ${MONKEY_TEST} PROPERTY COMPILE_OPTIONS "-Wall;-Werror;-Wpedantic")

target_link_libraries(${MONKEY_TEST} GTest::gtest_main)

target_include_directories(
    ${MONKEY_TEST}
    PRIVATE
    "${PROJECT_SOURCE_DIR}/include"
)

# Copy original MONKEY_SOURCES, but exclude `main.cc`;
# GoogleTest provides it's own main function for tests.
file(GLOB MONKEY_SOURCES_MINUS_MAIN "${PROJECT_SOURCE_DIR}/src/**/*.cc")
list(
    FILTER MONKEY_SOURCES_MINUS_MAIN
    EXCLUDE REGEX "${PROJECT_SOURCE_DIR}/src/main.cc"
)
target_sources(${MONKEY_TEST} PRIVATE ${MONKEY_SOURCES_MINUS_MAIN})

include(GoogleTest)
gtest_discover_tests(${MONKEY_TEST})

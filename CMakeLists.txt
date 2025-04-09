cmake_minimum_required(VERSION 3.10)

# Project name and version
project(rayinteract VERSION 0.1.5 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Define the library
file(GLOB_RECURSE RAYINTERACT_SOURCES src/*.cpp)
add_library(rayinteract STATIC ${RAYINTERACT_SOURCES})

# Include directories for the library
target_include_directories(rayinteract PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/src>
                                           $<INSTALL_INTERFACE:include/rayinteract>)

# Link Raylib
find_package(raylib REQUIRED)
target_link_libraries(rayinteract PUBLIC raylib)

# Installation rules
install(TARGETS rayinteract
    EXPORT rayinteractTargets
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
)

install(DIRECTORY src/
    DESTINATION include/rayinteract
    FILES_MATCHING PATTERN "*.hpp"
)

# Export the targets to a script
install(EXPORT rayinteractTargets
    FILE rayinteractTargets.cmake
    NAMESPACE rayinteract::
    DESTINATION lib/cmake/rayinteract
)

# Generate a Config.cmake file
include(CMakePackageConfigHelpers)
write_basic_package_version_file(
    "${CMAKE_CURRENT_BINARY_DIR}/rayinteractConfigVersion.cmake"
    VERSION ${PROJECT_VERSION}
    COMPATIBILITY SameMajorVersion
)

configure_package_config_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/cmake/rayinteractConfig.cmake.in"
    "${CMAKE_CURRENT_BINARY_DIR}/rayinteractConfig.cmake"
    INSTALL_DESTINATION lib/cmake/rayinteract
)

install(FILES
    "${CMAKE_CURRENT_BINARY_DIR}/rayinteractConfig.cmake"
    "${CMAKE_CURRENT_BINARY_DIR}/rayinteractConfigVersion.cmake"
    DESTINATION lib/cmake/rayinteract
)

# Instructions for usage
# 1. To build the project:
#    mkdir build && cd build
#    cmake ..
#    cmake --build .
# 2. To install the library:
#    cmake --install . --prefix /path/to/install
# 3. To use the library in another project:
#    find_package(rayinteract REQUIRED)
#    target_link_libraries(your_target PRIVATE rayinteract::rayinteract)
# rayinteract-config.cmake

cmake_minimum_required(VERSION 3.10)

# Default installation paths (can be overridden by package managers)
set(rayinteract_INCLUDE_DIR "" CACHE PATH "Directory for rayinteract headers")
set(rayinteract_LIB_DIR "" CACHE PATH "Directory for rayinteract library")

# Search for header and library locations
find_path(rayinteract_INCLUDE_DIR NAMES rayinteractlibs.h PATHS /usr/include /usr/local/include)
find_library(rayinteract_LIBRARY NAMES libraylib_interactive PATHS /usr/lib /usr/local/lib)

# If paths are not found, report an error
if(NOT rayinteract_INCLUDE_DIR OR NOT rayinteract_LIBRARY)
    message(FATAL_ERROR "rayinteract not found. Set rayinteract_INCLUDE_DIR and rayinteract_LIB_DIR.")
endif()

# Define imported target
add_library(rayinteract::rayinteract STATIC IMPORTED)
set_target_properties(rayinteract::rayinteract PROPERTIES
    IMPORTED_LOCATION "${rayinteract_LIBRARY}"
    INTERFACE_INCLUDE_DIRECTORIES "${rayinteract_INCLUDE_DIR}"
)

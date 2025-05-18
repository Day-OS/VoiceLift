cmake_minimum_required(VERSION 3.22.1)
project("cppshared_dummy")

add_library(${CMAKE_PROJECT_NAME} SHARED
        # List C/C++ source files with relative paths to this CMakeLists.txt.
        dummy.cpp)
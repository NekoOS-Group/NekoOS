cmake_minimum_required(VERSION 3.22)
project(NekoOS_kernel C)

set(CMAKE_C_STANDARD 11)

include_directories(.)
include_directories(include)
include_directories(include/neko)

add_executable(NekoOS_kernel
        include/neko/sys_config.h
        include/ansi.h
        include/assert.h)

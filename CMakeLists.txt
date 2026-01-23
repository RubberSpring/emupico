cmake_minimum_required(VERSION 3.15...4.1)

project(emupico VERSION 0.1
                  DESCRIPTION "PICO-8 emulator"
                  LANGUAGES CXX)

include(FindPkgConfig)

find_package(SDL2 REQUIRED)
find_package(Lua REQUIRED)
include_directories(emupico ${SDL2_INCLUDE_DIRS})
include_directories(emupico ${LUA_INCLUDE_DIRS})
include_directories(emupico include/)
include_directories(emupico extern/)

add_executable(emupico src/main.cpp src/dummy.cpp src/funcs.cpp)
target_link_libraries(emupico ${SDL2_LIBRARIES})
target_link_libraries(emupico ${LUA_LIBRARIES})
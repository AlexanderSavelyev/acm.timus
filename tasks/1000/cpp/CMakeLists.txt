cmake_minimum_required(VERSION 2.6)

project(task)

file (GLOB task_src "${CMAKE_CURRENT_SOURCE_DIR}/*.cpp" "${CMAKE_CURRENT_SOURCE_DIR}/*.h" )
message(STATUS "XX ${CMAKE_CURRENT_SOURCE_DIR}")

add_executable(task ${task_src})

if (UNIX)
	#set_target_properties(task PROPERTIES LINK_FLAGS "${LINK_FLAGS} -fno-strict-aliasing -lm -s 
   #  -Wl,--stack=67108864 -O2")
   set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -x c++ -std=c++11" )
endif()




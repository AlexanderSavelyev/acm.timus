cmake_minimum_required(VERSION 2.6)

project(task)

#file (GLOB task_src "${CMAKE_CURRENT_SOURCE_DIR}/test.cpp" "${CMAKE_CURRENT_SOURCE_DIR}/*.h" )

add_executable(task test.cpp)

if (UNIX)
	set_target_properties(task PROPERTIES LINK_FLAGS "${LINK_FLAGS} -fno-strict-aliasing ")
   set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11" )
endif()




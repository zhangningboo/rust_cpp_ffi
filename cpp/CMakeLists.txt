cmake_minimum_required(VERSION 3.16)
project(cpp)

# 设置 C++ 标准
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_C_COMPILER "/usr/bin/gcc")
set(CMAKE_CXX_COMPILER "/usr/bin/g++")
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# set(OpenCV_DIR $ENV{OpenCV_HOME}/lib/cmake/opencv4)
set(OpenCV_DIR /data/workspace/aky_vlm_project/3rd/opencv_4.9.0_x86/lib/cmake/opencv4)
find_package(OpenCV REQUIRED)
include_directories(${OpenCV_INCLUDE_DIRS})

include_directories(${CMAKE_SOURCE_DIR}/include)

# 生成静态库（或 SHARED 动态库）
add_library(${PROJECT_NAME} STATIC
    src/cpp_str.cpp
    src/cpp_number.cpp
    src/cpp_array.cpp
    src/cpp_map.cpp
    src/cpp_class.cpp
    src/cpp_segment.cpp
)
# 头文件路径（供 Rust FFI 使用）
target_include_directories(${PROJECT_NAME} PUBLIC ${CMAKE_SOURCE_DIR}/include)
target_link_libraries(${PROJECT_NAME} PRIVATE
    ${OpenCV_LIBS}
)
install(TARGETS ${PROJECT_NAME} DESTINATION .)


set(PROJECT_EXEC_NAME ${PROJECT_NAME}_bin)
add_executable(${PROJECT_EXEC_NAME} src/main.cpp
    src/cpp_str.cpp
    src/cpp_number.cpp
    src/cpp_array.cpp
    src/cpp_map.cpp
    src/cpp_class.cpp
    src/cpp_segment.cpp
)
target_include_directories(${PROJECT_EXEC_NAME} PUBLIC ${CMAKE_SOURCE_DIR}/include)
target_link_libraries(${PROJECT_EXEC_NAME} PRIVATE
    ${OpenCV_LIBS}
)
cmake_minimum_required(VERSION 3.16)
project(cpp)

# 设置 C++ 标准
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(FFMPEG_DIR $ENV{FFMPEG_HOME}/lib/cmake)
find_package(FFMPEG REQUIRED NO_DEFAULT_PATH)
include_directories(${FFMPEG_INCLUDE_DIRS})
link_directories(${FFMPEG_LIB_DIRS})

set(OpenCV_DIR $ENV{OpenCV_HOME}/lib/cmake/opencv4)
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
target_link_directories(${PROJECT_NAME} PUBLIC
    ${FFMPEG_LIB_DIRS}
)
target_link_libraries(${PROJECT_NAME} PUBLIC
    ${BM_LIBS}
    ${OpenCV_LIBS}
)
install(TARGETS ${PROJECT_NAME} DESTINATION ${CMAKE_SOURCE_DIR}/../cpp_lib)


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
target_link_directories(${PROJECT_EXEC_NAME} PUBLIC
    ${FFMPEG_LIB_DIRS}
)
target_link_libraries(${PROJECT_EXEC_NAME} PUBLIC
    ${OpenCV_LIBS}
)
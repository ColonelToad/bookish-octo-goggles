cmake_minimum_required(VERSION 3.10)
project(kips_ui_cpp)

set(CMAKE_CXX_STANDARD 17)

# Path to the Rust staticlib
set(RUST_LIB_PATH "${CMAKE_SOURCE_DIR}/target/release")
set(RUST_LIB "${RUST_LIB_PATH}/kips_ui.a")

# Include headers
include_directories(${CMAKE_SOURCE_DIR}/include)

# Add C++ source files
file(GLOB CPP_SOURCES src-cpp/*.cpp)

# Output executable
add_executable(kips_ui_cpp ${CPP_SOURCES})

# Link with Rust staticlib
target_link_libraries(kips_ui_cpp ${RUST_LIB})

# Optional: link SDL2 or other libs if needed
# find_package(SDL2 REQUIRED)
# target_include_directories(kips_ui_cpp PRIVATE ${SDL2_INCLUDE_DIRS})
# target_link_libraries(kips_ui_cpp ${SDL2_LIBRARIES})

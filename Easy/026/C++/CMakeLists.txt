cmake_minimum_required(VERSION 3.15)
project(MyProject)

# Check if /opt/homebrew exists
if(EXISTS "/opt/homebrew")
    message(STATUS "/opt/homebrew exists, configuring for macOS with Homebrew")
    include_directories("/opt/homebrew/include")
    link_directories("/opt/homebrew/lib")
else()
    message(STATUS "/opt/homebrew does not exist, skipping macOS Homebrew configuration")
endif()

include_directories(/usr/local/include)
link_directories(/usr/local/lib)

# # Find a package (example: OpenSSL)
# find_package(OpenSSL REQUIRED)
# include_directories(${OPENSSL_INCLUDE_DIR})
# link_directories(${OPENSSL_LIBRARIES})

# Add executable
add_executable(MyTarget src src/main.cpp)

# # Link libraries
# target_link_libraries(MyTarget ${OPENSSL_LIBRARIES})

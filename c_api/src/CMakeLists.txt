set(CMAKE_CXX_STANDARD 17)
add_library(zxing_c_api reader.cpp writer.cpp)
target_link_libraries(zxing_c_api ZXingCore)

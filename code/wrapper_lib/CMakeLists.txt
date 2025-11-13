# source files

#set(CMAKE_VERBOSE_MAKEFILE ON)
cmake_minimum_required(VERSION 2.6)
find_package(PkgConfig REQUIRED)
pkg_check_modules(GST REQUIRED gstreamer-1.0 gstreamer-base-1.0)


SET(HEADERS
../ScreamTx.h
../RtpQueue.h
)

SET(SRCS
../RtpQueue.cpp
../ScreamTx.cpp
../ScreamV2Tx.cpp
../ScreamV2TxStream.cpp
screamtxbw_plugin_wrapper.cpp
screamtx_plugin_wrapper.cpp
)

set(CMAKE_BUILD_TYPE Debug)

if(BUILD_SHARED_LIBS)
  set(CMAKE_CXX_FLAGS                " -DV2 -I.. ${CMAKE_CXX_FLAGS_INIT} ")
  add_library(scream SHARED ${SRCS})

else()
  set(CMAKE_CXX_FLAGS                " -DV2  -g  -I .. ${CMAKE_CXX_FLAGS_INIT} -fPIC")
  add_library(scream STATIC ${SRCS})
endif()

target_include_directories(scream PRIVATE
  ${GST_INCLUDE_DIRS}
)

target_link_directories(scream PRIVATE
  ${GST_LIBRARY_DIRS}
)
target_link_libraries(scream PRIVATE
  ${GST_LIBRARIES}
)


INCLUDE_DIRECTORIES(
${screamIncludes}
)

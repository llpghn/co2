# Install script for directory: /Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/patrickberendes/projects/o2monitor/handler/i2c/target/debug/build/paho-mqtt-sys-26ddd79901cf2c1a/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/doc/Eclipse Paho C/samples" TYPE FILE FILES
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/src/samples/MQTTAsync_publish.c"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/src/samples/MQTTAsync_publish_time.c"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/src/samples/MQTTAsync_subscribe.c"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/src/samples/MQTTClient_publish.c"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/src/samples/MQTTClient_publish_async.c"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/src/samples/MQTTClient_subscribe.c"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/src/samples/paho_c_pub.c"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/src/samples/paho_c_sub.c"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/src/samples/paho_cs_pub.c"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/src/samples/paho_cs_sub.c"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/src/samples/pubsub_opts.c"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/doc/Eclipse Paho C" TYPE FILE FILES
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/CONTRIBUTING.md"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/epl-v20"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/edl-v10"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/README.md"
    "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/notice.html"
    )
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/Users/patrickberendes/projects/o2monitor/handler/i2c/target/debug/build/paho-mqtt-sys-26ddd79901cf2c1a/out/build/src/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/Users/patrickberendes/projects/o2monitor/handler/i2c/target/debug/build/paho-mqtt-sys-26ddd79901cf2c1a/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")

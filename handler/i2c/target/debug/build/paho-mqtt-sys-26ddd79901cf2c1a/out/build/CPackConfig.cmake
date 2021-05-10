# This file will be configured to contain variables for CPack. These variables
# should be set in the CMake list file of the project before CPack module is
# included. The list of available CPACK_xxx variables and their associated
# documentation may be obtained using
#  cpack --help-variable-list
#
# Some variables are common to all generators (e.g. CPACK_PACKAGE_NAME)
# and some are specific to a generator
# (e.g. CPACK_NSIS_EXTRA_INSTALL_COMMANDS). The generator specific variables
# usually begin with CPACK_<GENNAME>_xxxx.


set(CPACK_BUILD_SOURCE_DIRS "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c;/Users/patrickberendes/projects/o2monitor/handler/i2c/target/debug/build/paho-mqtt-sys-26ddd79901cf2c1a/out/build")
set(CPACK_CMAKE_GENERATOR "Unix Makefiles")
set(CPACK_COMPONENT_UNSPECIFIED_HIDDEN "TRUE")
set(CPACK_COMPONENT_UNSPECIFIED_REQUIRED "TRUE")
set(CPACK_DEFAULT_PACKAGE_DESCRIPTION_FILE "/usr/local/Cellar/cmake/3.20.2/share/cmake/Templates/CPack.GenericDescription.txt")
set(CPACK_DEFAULT_PACKAGE_DESCRIPTION_SUMMARY "Eclipse Paho C built using CMake")
set(CPACK_GENERATOR "TGZ")
set(CPACK_INSTALL_CMAKE_PROJECTS "/Users/patrickberendes/projects/o2monitor/handler/i2c/target/debug/build/paho-mqtt-sys-26ddd79901cf2c1a/out/build;Eclipse Paho C;ALL;/")
set(CPACK_INSTALL_PREFIX "/Users/patrickberendes/projects/o2monitor/handler/i2c/target/debug/build/paho-mqtt-sys-26ddd79901cf2c1a/out")
set(CPACK_MODULE_PATH "/Users/patrickberendes/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/312270c/paho-mqtt-sys/paho.mqtt.c/cmake/modules")
set(CPACK_NSIS_DISPLAY_NAME "Eclipse-Paho-MQTT-C 1.3.8")
set(CPACK_NSIS_INSTALLER_ICON_CODE "")
set(CPACK_NSIS_INSTALLER_MUI_ICON_CODE "")
set(CPACK_NSIS_INSTALL_ROOT "$PROGRAMFILES")
set(CPACK_NSIS_PACKAGE_NAME "Eclipse-Paho-MQTT-C 1.3.8")
set(CPACK_NSIS_UNINSTALL_NAME "Uninstall")
set(CPACK_OSX_SYSROOT "/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk")
set(CPACK_OUTPUT_CONFIG_FILE "/Users/patrickberendes/projects/o2monitor/handler/i2c/target/debug/build/paho-mqtt-sys-26ddd79901cf2c1a/out/build/CPackConfig.cmake")
set(CPACK_PACKAGE_DEFAULT_LOCATION "/")
set(CPACK_PACKAGE_DESCRIPTION_FILE "/usr/local/Cellar/cmake/3.20.2/share/cmake/Templates/CPack.GenericDescription.txt")
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "Eclipse Paho C built using CMake")
set(CPACK_PACKAGE_FILE_NAME "Eclipse-Paho-MQTT-C-1.3.8-Darwin")
set(CPACK_PACKAGE_INSTALL_DIRECTORY "Eclipse-Paho-MQTT-C 1.3.8")
set(CPACK_PACKAGE_INSTALL_REGISTRY_KEY "Eclipse-Paho-MQTT-C 1.3.8")
set(CPACK_PACKAGE_NAME "Eclipse-Paho-MQTT-C")
set(CPACK_PACKAGE_RELOCATABLE "true")
set(CPACK_PACKAGE_VENDOR "Eclipse Paho")
set(CPACK_PACKAGE_VERSION "1.3.8")
set(CPACK_PACKAGE_VERSION_MAJOR "1")
set(CPACK_PACKAGE_VERSION_MINOR "3")
set(CPACK_PACKAGE_VERSION_PATCH "8")
set(CPACK_RESOURCE_FILE_LICENSE "/usr/local/Cellar/cmake/3.20.2/share/cmake/Templates/CPack.GenericLicense.txt")
set(CPACK_RESOURCE_FILE_README "/usr/local/Cellar/cmake/3.20.2/share/cmake/Templates/CPack.GenericDescription.txt")
set(CPACK_RESOURCE_FILE_WELCOME "/usr/local/Cellar/cmake/3.20.2/share/cmake/Templates/CPack.GenericWelcome.txt")
set(CPACK_SET_DESTDIR "OFF")
set(CPACK_SOURCE_GENERATOR "TBZ2;TGZ;TXZ;TZ")
set(CPACK_SOURCE_OUTPUT_CONFIG_FILE "/Users/patrickberendes/projects/o2monitor/handler/i2c/target/debug/build/paho-mqtt-sys-26ddd79901cf2c1a/out/build/CPackSourceConfig.cmake")
set(CPACK_SOURCE_RPM "OFF")
set(CPACK_SOURCE_TBZ2 "ON")
set(CPACK_SOURCE_TGZ "ON")
set(CPACK_SOURCE_TXZ "ON")
set(CPACK_SOURCE_TZ "ON")
set(CPACK_SOURCE_ZIP "OFF")
set(CPACK_SYSTEM_NAME "Darwin")
set(CPACK_THREADS "1")
set(CPACK_TOPLEVEL_TAG "Darwin")
set(CPACK_WIX_SIZEOF_VOID_P "8")

if(NOT CPACK_PROPERTIES_FILE)
  set(CPACK_PROPERTIES_FILE "/Users/patrickberendes/projects/o2monitor/handler/i2c/target/debug/build/paho-mqtt-sys-26ddd79901cf2c1a/out/build/CPackProperties.cmake")
endif()

if(EXISTS ${CPACK_PROPERTIES_FILE})
  include(${CPACK_PROPERTIES_FILE})
endif()

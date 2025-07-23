## How to build

### Prepare

* run verovio/tools/get_git_commit.sh
* modify verovio/cmake/CMakeLists.txt, BUILD_AS_LIBRARY changed to static

### All Platform

CMake 3.22 and later is needed.

### Windows

Windows need Visual Studio 2022 and later

### Linux

Linux need Ninja

### Android

ANDROID_NDK_HOME environment variable need to be set to the NDK path
CC environment variable need to be set to the NDK path
CXX environment variable need to be set to the NDK path

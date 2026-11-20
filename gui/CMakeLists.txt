cmake_minimum_required(VERSION 3.16)
project(rmouse-gui LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION_RELEASE ON)

if(NOT MSVC)
    add_compile_options("$<$<CONFIG:Release>:-O3;-fPIE>")
    add_link_options("$<$<CONFIG:Release>:-pie;-s>")
endif()

find_package(Qt6 REQUIRED COMPONENTS Core Gui Widgets Sql)

set(SOURCES
    src/main.cpp
    src/MainWindow.h
    src/MainWindow.cpp
    src/SimulatedKeystrokesDialog.h
    src/SimulatedKeystrokesDialog.cpp
    src/GlobalSettingsDialog.h
    src/GlobalSettingsDialog.cpp
    src/ProcessSelectorDialog.h
    src/ProcessSelectorDialog.cpp
    src/Database.h
    src/Database.cpp
    src/Localization.h
    src/Localization.cpp
    resources/resources.qrc
)

add_executable(rmouse-gui ${SOURCES})

if(WIN32)
    set_target_properties(rmouse-gui PROPERTIES WIN32_EXECUTABLE TRUE)
endif()

target_link_libraries(rmouse-gui PRIVATE
    Qt6::Core
    Qt6::Gui
    Qt6::Widgets
    Qt6::Sql
)

install(TARGETS rmouse-gui DESTINATION bin)

// src-cpp/launcher_bridge.cpp
#include "launcher_bridge.h"

extern "C" {
    void launch_file_explorer() {
        launch_app_file_explorer();
    }

    void launch_terminal() {
        launch_app_terminal();
    }

    void launch_ide() {
        launch_app_ide();
    }

    void launch_text_editor() {
        launch_app_text_editor();
    }

    void launch_calendar() {
        launch_app_calendar();
    }

    void launch_maps() {
        launch_app_maps();
    }

    void launch_notes() {
        launch_app_notes();
    }

    void launch_todo_list(){
        launch_app_todo_list();
    }

    void launch_writer(const char* filename) {
        launch_app_writer(filename);
    }

    void launch_calc(const char* filename) {
        launch_app_calc(filename);
    }

    void launch_impress(const char* filename) {
        launch_app_impress(filename);
    }
}

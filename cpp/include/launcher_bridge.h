// include/launcher_bridge.h
#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// These represent the basic apps to launch from C++
void launch_file_explorer();
void launch_terminal();
void launch_ide();
void launch_text_editor();
void launch_calendar();
void launch_maps();
void launch_notes();
void launch_todo_list();

// LibreOffice launches with optional filenames
void launch_writer(const char* filename);   // Pass NULL if no file
void launch_calc(const char* filename);
void launch_impress(const char* filename);

#ifdef __cplusplus
}
#endif
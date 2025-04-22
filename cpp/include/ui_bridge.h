#ifndef UI_BRIDGE_H
#define UI_BRIDGE_H

#ifdef __cplusplus
extern "C" {
#endif

// Call to initialize the UI (e.g., set up SDL, fonts, etc.)
void ui_initialize();

// Renders the welcome screen
void ui_draw_welcome_screen();

// Renders the main menu with a given selected index
void ui_draw_main_menu(int selected_index);

// Tells the UI to shut down gracefully if needed
void ui_shutdown();

#ifdef __cplusplus
}
#endif

#endif // UI_BRIDGE_H
// cpp/src/ui_bridge.cpp
#include "ui_bridge.h"
#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <SDL2/SDL_ttf.h>
#include <vector>
#include <string>

// --- Color Scheme ---
SDL_Color COLOR_BG       = {0x02, 0x2F, 0x2A, 0xFF}; // Background
SDL_Color COLOR_UNSELECT = {0x0E, 0x6C, 0x79, 0xFF}; // Unselected menu items
SDL_Color COLOR_SELECT   = {0x0D, 0xCE, 0xEB, 0xFF}; // Selected menu item
SDL_Color COLOR_TEXT     = {0xFF, 0xFF, 0xFF, 0xFF}; // Text
SDL_Color COLOR_OVERLAY  = {0xB1, 0xE6, 0xED, 0x80}; // Optional overlay

extern SDL_Renderer* gRenderer; // Global renderer from your main setup
extern TTF_Font* gFont;         // Global font from your main setup

void draw_text_centered(const std::string& text, int y, SDL_Color color) {
    SDL_Surface* surface = TTF_RenderText_Blended(gFont, text.c_str(), color);
    SDL_Texture* texture = SDL_CreateTextureFromSurface(gRenderer, surface);
    int w = surface->w, h = surface->h;
    SDL_Rect dst = { 400 - w / 2, y, w, h };
    SDL_FreeSurface(surface);
    SDL_RenderCopy(gRenderer, texture, nullptr, &dst);
    SDL_DestroyTexture(texture);
}

void draw_button_row(const std::vector<std::string>& labels, int selected) {
    int spacing = 800 / labels.size();
    int y = 440, box_h = 40;
    for (size_t i = 0; i < labels.size(); ++i) {
        int x = i * spacing;
        SDL_Rect rect = {x, y, spacing, box_h};

        SDL_SetRenderDrawColor(gRenderer,
            (i == selected ? COLOR_SELECT.r : COLOR_UNSELECT.r),
            (i == selected ? COLOR_SELECT.g : COLOR_UNSELECT.g),
            (i == selected ? COLOR_SELECT.b : COLOR_UNSELECT.b), 255);
        SDL_RenderFillRect(gRenderer, &rect);

        draw_text_centered(labels[i], y + 10, COLOR_TEXT); // rough center
    }
}

void draw_welcome_screen(int selected_index) {
    SDL_SetRenderDrawColor(gRenderer, COLOR_BG.r, COLOR_BG.g, COLOR_BG.b, 255);
    SDL_RenderClear(gRenderer);

    draw_text_centered("WELCOME, USER!", 50, COLOR_TEXT);

    SDL_Surface* image = IMG_Load("assets/sit.png");
    SDL_Texture* texture = SDL_CreateTextureFromSurface(gRenderer, image);
    SDL_Rect center = { 200, 100, 400, 240 }; // example placement
    SDL_FreeSurface(image);
    SDL_RenderCopy(gRenderer, texture, nullptr, &center);
    SDL_DestroyTexture(texture);

    draw_button_row({"APPS", "PROFILE", "SETTINGS"}, selected_index);

    SDL_RenderPresent(gRenderer);
}

void draw_main_menu(const std::vector<std::string>& items, int selected) {
    SDL_SetRenderDrawColor(gRenderer, COLOR_BG.r, COLOR_BG.g, COLOR_BG.b, 255);
    SDL_RenderClear(gRenderer);

    for (size_t i = 0; i < items.size(); ++i) {
        int y = 100 + static_cast<int>(i) * 60;
        SDL_Color color = (i == selected) ? COLOR_SELECT : COLOR_UNSELECT;
        draw_text_centered(items[i], y, color);
    }

    SDL_RenderPresent(gRenderer);
}
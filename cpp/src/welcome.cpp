#include "welcome.h"
#include <SDL.h>
#include <SDL_ttf.h>
#include <iostream>

extern SDL_Renderer* renderer;
extern TTF_Font* font;

extern "C" void ui_draw_welcome_screen() {
    std::cout << "[CPP] Drawing welcome screen..." << std::endl;

    // Clear with background color (#022F2A)
    SDL_SetRenderDrawColor(renderer, 0x02, 0x2F, 0x2A, 255);
    SDL_RenderClear(renderer);

    // Draw welcome text
    SDL_Color white = {255, 255, 255};
    SDL_Surface* surface = TTF_RenderText_Blended(font, "WELCOME, USER!", white);
    SDL_Texture* texture = SDL_CreateTextureFromSurface(renderer, surface);
    SDL_Rect dst = { (800 - surface->w) / 2, 50, surface->w, surface->h };
    SDL_RenderCopy(renderer, texture, nullptr, &dst);
    SDL_FreeSurface(surface);
    SDL_DestroyTexture(texture);

    // Bottom taskbar boxes
    const char* labels[3] = {"APPS", "PROFILE", "SETTINGS"};
    int spacing = 800 / 3;
    int y = 440, box_height = 40;

    for (int i = 0; i < 3; ++i) {
        int x = i * spacing;

        // Box background: #0E6C79
        SDL_SetRenderDrawColor(renderer, 0x0E, 0x6C, 0x79, 255);
        SDL_Rect box = {x, y, spacing, box_height};
        SDL_RenderFillRect(renderer, &box);

        // Text
        SDL_Surface* label_surface = TTF_RenderText_Blended(font, labels[i], white);
        SDL_Texture* label_texture = SDL_CreateTextureFromSurface(renderer, label_surface);
        SDL_Rect text_rect = {
            x + (spacing - label_surface->w) / 2,
            y + (box_height - label_surface->h) / 2,
            label_surface->w,
            label_surface->h
        };
        SDL_RenderCopy(renderer, label_texture, nullptr, &text_rect);
        SDL_FreeSurface(label_surface);
        SDL_DestroyTexture(label_texture);
    }

    SDL_RenderPresent(renderer);
}
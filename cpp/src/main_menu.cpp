#include "main_menu.h"
#include <SDL.h>
#include <SDL_ttf.h>
#include <iostream>

extern SDL_Renderer* renderer;
extern TTF_Font* font;

extern "C" void ui_draw_main_menu(int selected_index) {
    std::cout << "[CPP] Main Menu selected index: " << selected_index << std::endl;

    SDL_SetRenderDrawColor(renderer, 0x02, 0x2F, 0x2A, 255); // background
    SDL_RenderClear(renderer);

    const char* items[5] = {
        "Music Player", "Calendar", "Camera", "Files", "Back"
    };

    for (int i = 0; i < 5; ++i) {
        SDL_Color text_color = {255, 255, 255};
        SDL_Surface* text_surf = TTF_RenderText_Blended(font, items[i], text_color);
        SDL_Texture* text_tex = SDL_CreateTextureFromSurface(renderer, text_surf);

        SDL_Rect box = {100, 80 + i * 60, 600, 50};

        // Highlight if selected
        if (i == selected_index) {
            SDL_SetRenderDrawColor(renderer, 0x0D, 0xCE, 0xEB, 255); // selected
        } else {
            SDL_SetRenderDrawColor(renderer, 0x0E, 0x6C, 0x79, 255); // unselected
        }
        SDL_RenderFillRect(renderer, &box);

        SDL_Rect text_dst = {
            box.x + (box.w - text_surf->w) / 2,
            box.y + (box.h - text_surf->h) / 2,
            text_surf->w,
            text_surf->h
        };
        SDL_RenderCopy(renderer, text_tex, nullptr, &text_dst);

        SDL_FreeSurface(text_surf);
        SDL_DestroyTexture(text_tex);
    }

    SDL_RenderPresent(renderer);
}
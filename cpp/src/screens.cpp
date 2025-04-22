#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>
#include <iostream>
#include <vector>
#include <string>
#include <cstdlib>

extern SDL_Renderer* renderer;
extern TTF_Font* font;

void drawText(const std::string& text, int x, int y, SDL_Color color) {
    SDL_Surface* surface = TTF_RenderText_Blended(font, text.c_str(), color);
    SDL_Texture* texture = SDL_CreateTextureFromSurface(renderer, surface);
    SDL_Rect dst = { x, y, surface->w, surface->h };
    SDL_RenderCopy(renderer, texture, nullptr, &dst);
    SDL_FreeSurface(surface);
    SDL_DestroyTexture(texture);
}

void drawRoundedRect(SDL_Rect rect, SDL_Color color) {
    SDL_SetRenderDrawColor(renderer, color.r, color.g, color.b, color.a);
    SDL_RenderFillRect(renderer, &rect);
}

void drawCircle(int cx, int cy, int radius, SDL_Color color) {
    SDL_SetRenderDrawColor(renderer, color.r, color.g, color.b, color.a);
    for (int w = 0; w < radius * 2; w++) {
        for (int h = 0; h < radius * 2; h++) {
            int dx = radius - w;
            int dy = radius - h;
            if ((dx*dx + dy*dy) <= (radius * radius)) {
                SDL_RenderDrawPoint(renderer, cx + dx, cy + dy);
            }
        }
    }
}

extern "C" void ui_draw_app_list(int page) {
    SDL_SetRenderDrawColor(renderer, 0x02, 0x2F, 0x2A, 255); // background
    SDL_RenderClear(renderer);

    SDL_Color light = { 0x0D, 0xCE, 0xEB, 255 };
    SDL_Color dark = { 0x0E, 0x6C, 0x79, 255 };
    SDL_Color white = { 255, 255, 255, 255 };

    std::vector<std::string> apps = {
        "Weather", "Audio Player", "Video Player", "Files",
        "Terminal", "Thonny", "Text File", "Calendar",
        "Maps", "Notes", "Todo List", "Writer", 
        "Calc", "Impress", "Back"
    };

    const int appsPerPage = 12;
    int start = page * appsPerPage;
    int end = std::min(start + appsPerPage, (int)apps.size());

    int box_w = 180;
    int box_h = 80;
    int spacing = 20;
    int cols = 4;
    int rows = 3;
    int offset_x = 40;
    int offset_y = 40;

    for (int i = start; i < end; ++i) {
        int idx = i - start;
        int col = idx % cols;
        int row = idx / cols;
        int x = offset_x + col * (box_w + spacing);
        int y = offset_y + row * (box_h + spacing);

        SDL_Rect rect = { x, y, box_w, box_h };
        drawRoundedRect(rect, dark);
        drawText(apps[i], x + 20, y + 25, white);
    }

    SDL_RenderPresent(renderer);
}

extern "C" void ui_draw_settings_screen() {
    SDL_SetRenderDrawColor(renderer, 0x02, 0x2F, 0x2A, 255);
    SDL_RenderClear(renderer);

    SDL_Color textColor = {255, 255, 255, 255};
    std::vector<std::string> items = {
        "Display", "Sound", "Wi-Fi", "Bluetooth", "Storage"
    };

    int y = 60;
    for (const auto& item : items) {
        SDL_Rect box = { 40, y, 720, 60 };
        drawRoundedRect(box, {0x0E, 0x6C, 0x79, 255});
        drawText(item, 60, y + 15, textColor);
        y += 80;
    }

    SDL_RenderPresent(renderer);
}

extern "C" void ui_draw_profile_screen() {
    SDL_SetRenderDrawColor(renderer, 0x02, 0x2F, 0x2A, 255);
    SDL_RenderClear(renderer);

    SDL_Color textColor = {255, 255, 255, 255};

    drawCircle(200, 180, 60, {0xB1, 0xE6, 0xED, 255});
    drawText("User 1", 175, 260, textColor);

    drawCircle(500, 180, 40, {0xB1, 0xE6, 0xED, 255});
    drawText("+", 490, 160, textColor);

    SDL_RenderPresent(renderer);
}
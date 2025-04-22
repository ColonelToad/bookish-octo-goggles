import pygame
import sys
import subprocess
import os
import time

# Screen size and colors
WIDTH, HEIGHT = 800, 480
BG_COLOR = (0x02, 0x2F, 0x2A)
SELECTED_COLOR = (0x0D, 0xCE, 0xEB)
UNSELECTED_COLOR = (0x0E, 0x6C, 0x79)
TEXT_COLOR = (255, 255, 255)
OVERLAY_COLOR = (0xB1, 0xE6, 0xED)

pygame.init()
screen = pygame.display.set_mode((WIDTH, HEIGHT))
pygame.display.set_caption("Wearable UI")
font = pygame.font.SysFont("Arial", 24)
clock = pygame.time.Clock()

# Sleep tracking
SLEEP_TIMEOUT = 30  # seconds
last_input_time = time.time()
sleep_mode = False
previous_screen = None
sleep_frames = []
sleep_frame_index = 0
sleep_frame_timer = 0

# Load sleep GIF frames
for i in range(6):  # Adjust count if needed
    frame_path = f"../assets/sleep/frame_{i}.png"
    if os.path.exists(frame_path):
        frame = pygame.image.load(frame_path)
        sleep_frames.append(pygame.transform.scale(frame, (200, 200)))

# Page states
SCREEN_WELCOME = "welcome"
SCREEN_MAINMENU = "mainmenu"
SCREEN_PROFILE = "profile"
SCREEN_SETTINGS = "settings"
SCREEN_APPGRID = "appgrid"
current_screen = SCREEN_WELCOME

# Helpers
def draw_text(text, x, y, color=TEXT_COLOR, center=False):
    surface = font.render(text, True, color)
    rect = surface.get_rect()
    rect.center = (x, y) if center else (x, y)
    screen.blit(surface, rect)

def draw_button(x, y, w, h, label, selected=False):
    color = SELECTED_COLOR if selected else UNSELECTED_COLOR
    pygame.draw.rect(screen, color, (x, y, w, h), border_radius=10)
    text_surface = font.render(label, True, TEXT_COLOR)
    text_rect = text_surface.get_rect(center=(x + w // 2, y + h // 2))
    screen.blit(text_surface, text_rect)

def draw_circle(x, y, r, color):
    pygame.draw.circle(screen, color, (x, y), r)

def draw_sleep_screen():
    global sleep_frame_index, sleep_frame_timer
    screen.fill(BG_COLOR)

    if sleep_frames:
        now = pygame.time.get_ticks()
        if now - sleep_frame_timer > 100:
            sleep_frame_index = (sleep_frame_index + 1) % len(sleep_frames)
            sleep_frame_timer = now

        frame = sleep_frames[sleep_frame_index]
        rect = frame.get_rect(center=(WIDTH // 2, HEIGHT // 2 - 40))
        screen.blit(frame, rect)

    draw_text("Going to sleep...", WIDTH // 2, HEIGHT - 80, center=True)

# Pages
def draw_welcome():
    screen.fill(BG_COLOR)
    draw_text("WELCOME, USER!", WIDTH // 2, 40, center=True)

    pygame.draw.rect(screen, OVERLAY_COLOR, (WIDTH//2 - 100, HEIGHT//2 - 100, 200, 200), border_radius=20)
    image = pygame.image.load("../assets/knightro.png")
    image = pygame.transform.scale(image, (100, 100))
    rect = image.get_rect(center=(WIDTH // 2, HEIGHT // 2))
    screen.blit(image, rect)

    draw_button(50, HEIGHT - 80, 200, 60, "APPS")
    draw_button(300, HEIGHT - 80, 200, 60, "PROFILE")
    draw_button(550, HEIGHT - 80, 200, 60, "SETTINGS")

def draw_main_menu():
    screen.fill(BG_COLOR)
    menu_items = ["Music Player", "Calendar", "Camera", "Files", "Back"]
    for i, item in enumerate(menu_items):
        draw_button(100, 50 + i * 70, 600, 50, item)

def draw_settings():
    screen.fill(BG_COLOR)
    settings = ["User Info", "Theme", "Security", "About", "Back"]
    for i, item in enumerate(settings):
        draw_button(100, 50 + i * 70, 600, 50, item)

def draw_profile():
    screen.fill(BG_COLOR)
    draw_circle(200, 180, 60, OVERLAY_COLOR)
    draw_text("User1", 175, 260)
    draw_circle(500, 180, 40, OVERLAY_COLOR)
    draw_text("+", 490, 160)

def draw_app_grid():
    screen.fill(BG_COLOR)
    apps = [
        ("Weather", ["cargo", "run", "--bin", "weather"]),
        ("Audio", ["cargo", "run", "--bin", "audio"]),
        ("Video", ["cargo", "run", "--bin", "video"]),
        ("Files", ["pcmanfm"]),
        ("Terminal", ["lxterminal"]),
        ("Text", ["mousepad"]),
        ("Calendar", ["flatpak", "run", "org.gnome.Calendar"]),
        ("Notes", ["flatpak", "run", "com.github.flxzt.rnote"]),
        ("Maps", ["flatpak", "run", "org.gnome.Maps"]),
        ("ToDo", ["flatpak", "run", "io.github.mrvladus.List"]),
        ("Writer", ["libreoffice", "--writer"]),
        ("Calc", ["libreoffice", "--calc"])
    ]

    box_w, box_h = 160, 80
    spacing = 20
    cols = 4
    for i, (label, cmd) in enumerate(apps):
        col = i % cols
        row = i // cols
        x = 40 + col * (box_w + spacing)
        y = 40 + row * (box_h + spacing)
        draw_button(x, y, box_w, box_h, label)

    return apps

# Main loop
apps = []
while True:
    screen.fill(BG_COLOR)

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            sys.exit()

        # Wake from sleep mode on any interaction
        if sleep_mode:
            sleep_mode = False
            current_screen = previous_screen
            last_input_time = time.time()
            continue

        if event.type == pygame.MOUSEBUTTONDOWN:
            last_input_time = time.time()

            x, y = event.pos
            if current_screen == SCREEN_WELCOME:
                if 50 <= x <= 250 and HEIGHT - 80 <= y <= HEIGHT - 20:
                    current_screen = SCREEN_APPGRID
                elif 300 <= x <= 500 and HEIGHT - 80 <= y <= HEIGHT - 20:
                    current_screen = SCREEN_PROFILE
                elif 550 <= x <= 750 and HEIGHT - 80 <= y <= HEIGHT - 20:
                    current_screen = SCREEN_SETTINGS
            elif current_screen == SCREEN_APPGRID:
                apps = draw_app_grid()
                box_w, box_h = 160, 80
                spacing = 20
                for i, (_, cmd) in enumerate(apps):
                    col = i % 4
                    row = i // 4
                    x0 = 40 + col * (box_w + spacing)
                    y0 = 40 + row * (box_h + spacing)
                    if x0 <= x <= x0 + box_w and y0 <= y <= y0 + box_h:
                        subprocess.Popen(cmd)
            elif current_screen in [SCREEN_SETTINGS, SCREEN_PROFILE]:
                current_screen = SCREEN_WELCOME

    # Check for inactivity timeout
    if not sleep_mode and time.time() - last_input_time > SLEEP_TIMEOUT:
        sleep_mode = True
        previous_screen = current_screen

    # Draw UI
    if sleep_mode:
        draw_sleep_screen()
    elif current_screen == SCREEN_WELCOME:
        draw_welcome()
    elif current_screen == SCREEN_MAINMENU:
        draw_main_menu()
    elif current_screen == SCREEN_SETTINGS:
        draw_settings()
    elif current_screen == SCREEN_PROFILE:
        draw_profile()
    elif current_screen == SCREEN_APPGRID:
        apps = draw_app_grid()

    pygame.display.flip()
    clock.tick(60)
import pigpio
import time

# Encoder Pins
CLK = 17
DT = 22
SW = 27

# Messages
messages = [
    "Message 1: Initial State",
    "Message 2: First Click",
    "Message 3: Second Click",
    "Message 4: Third Click",
    "Message 5: Fourth Click",
    "Message 6: Fifth Click",
    "Message 7: Sixth Click",
    "Message 8: Seventh Click",
    "Message 9: Eighth Click"
]

# Scroller Options
scroller_options = ["Option A", "Option B", "Option C", "Option D"]

# Variables
current_position = 0
current_message_index = 0
current_option_index = 0
selected_option = None
last_clk_state = 1  # Default state

# Initialize pigpio
pi = pigpio.pi()
if not pi.connected:
    print("Failed to connect to pigpio daemon.")
    exit()

# Set up encoder pins as inputs with pull-ups
pi.set_mode(CLK, pigpio.INPUT)
pi.set_mode(DT, pigpio.INPUT)
pi.set_mode(SW, pigpio.INPUT)

pi.set_pull_up_down(CLK, pigpio.PUD_UP)
pi.set_pull_up_down(DT, pigpio.PUD_UP)
pi.set_pull_up_down(SW, pigpio.PUD_UP)

# Rotary encoder handler
def handle_encoder(gpio, level, tick):
    global current_position, last_clk_state, current_message_index, current_option_index

    clk_state = pi.read(CLK)
    dt_state = pi.read(DT)

    if clk_state != last_clk_state:  
        if dt_state != clk_state:  # Clockwise
            current_position += 1
            direction = "Clockwise"
        else:  # Counter-clockwise
            current_position -= 1
            direction = "Counter-clockwise"

        current_message_index = current_position % len(messages)
        current_option_index = current_position % len(scroller_options)

        print(f"{direction}: {messages[current_message_index]}, Option: {scroller_options[current_option_index]}")
    
    last_clk_state = clk_state

# Register callbacks for both CLK and DT pins
pi.callback(CLK, pigpio.EITHER_EDGE, handle_encoder)
pi.callback(DT, pigpio.EITHER_EDGE, handle_encoder)

# Encoder button handling
def handle_button(gpio, level, tick):
    global selected_option

    if level == 0:  # Button pressed
        time.sleep(0.05)  # Debounce
        if pi.read(SW) == 0:  # Ensure it's still pressed
            selected_option = scroller_options[current_option_index]
            print(f"Encoder Button Pressed! Selected: {selected_option}")
            while pi.read(SW) == 0:
                pass  # Wait for release

# Register callback for button press
pi.callback(SW, pigpio.FALLING_EDGE, handle_button)

try:
    while True:
        time.sleep(0.01)  # Small delay to reduce CPU usage

except KeyboardInterrupt:
    print("Exiting...")
    pi.stop()  # Clean up pigpio resources
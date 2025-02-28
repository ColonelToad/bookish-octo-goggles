import RPi.GPIO as GPIO
import time
from datetime import datetime

# Define GPIO pins for each button
button_pins = [17, 27, 22, 23] 

# Initialize button press counters
button_press_counts = [0] * len(button_pins)

# Define scroller options for button 2
scroller_options = ["Option A", "Option B", "Option C", "Option D"]
current_option_index_2 = 0
selected_option_2 = None 

# Set up GPIO
GPIO.setmode(GPIO.BCM)
for pin in button_pins:
    GPIO.setup(pin, GPIO.IN, pull_up_down=GPIO.PUD_UP)

try:
    while True:
        for i, pin in enumerate(button_pins):
            if GPIO.input(pin) == 0:
                time.sleep(0.2)  # Debounce delay

                if i == 0:  # Button 1 (Multi-press select)
                    button_press_counts[i] += 1
                    if button_press_counts[i] == 1:
                        print(f"Button {i+1} pressed")
                    elif button_press_counts[i] == 2:
                        print(f"Current time: {datetime.now().strftime('%H:%M:%S')}")
                    elif button_press_counts[i] == 3:
                        print(f"Current date: {datetime.now().strftime('%Y-%m-%d')}")
                    if button_press_counts[i] > 3:
                        button_press_counts[i] = 0

                elif i == 1:  # Button 2 (Scroller)
                    current_option_index_2 = (current_option_index_2 + 1) % len(scroller_options)
                    print(f"Current option: {scroller_options[current_option_index_2]}") 

                elif i == 2:  # Button 3 (Select current option for Button 2)
                    if selected_option_2 is None:
                        selected_option_2 = scroller_options[current_option_index_2]
                        print(f"Selected option: {selected_option_2}")

                else:  # Button 4 (Single press)
                    print(f"Button {i+1} pressed")

except KeyboardInterrupt:
    print("Exiting...")
    GPIO.cleanup()
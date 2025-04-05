use rusb::{Context, DeviceList};
use notify_rust::Notification;
use std::{thread, time};

fn get_connected_usb_ids() -> Vec<String> {
    let context = Context::new().unwrap();
    let devices = DeviceList::new().unwrap();

    devices
        .iter()
        .map(|device| {
            let desc = device.device_descriptor().unwrap();
            format!("{:04x}:{:04x}", desc.vendor_id(), desc.product_id())
        })
        .collect()
}

fn main() {
    println!("Listening for USB device changes...");

    let mut known = get_connected_usb_ids();

    loop {
        thread::sleep(time::Duration::from_secs(1));

        let current = get_connected_usb_ids();

        for id in &current {
            if !known.contains(id) {
                println!("USB device plugged in: {}", id);
                Notification::new()
                    .summary("USB Device Connected")
                    .body(&format!("Device {} connected", id))
                    .show()
                    .unwrap();
            }
        }

        for id in &known {
            if !current.contains(id) {
                println!("USB device removed: {}", id);
                Notification::new()
                    .summary("USB Device Disconnected")
                    .body(&format!("Device {} removed", id))
                    .show()
                    .unwrap();
            }
        }

        known = current;
    }
}

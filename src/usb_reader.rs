use rusb::{Context, DeviceList};
use std::{thread, time};

fn get_usb_ids() -> Vec<String> {
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
    println!("Monitoring USB ports for HOLOTAPE...");

    let mut previous = get_usb_ids();

    loop {
        thread::sleep(time::Duration::from_secs(1));
        let current = get_usb_ids();

        for id in &current {
            if !previous.contains(id) {
                println!("HOLOTAPE DETECTED");
            }
        }

        previous = current;
    }
}

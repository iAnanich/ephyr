//! Audio Redirect
use cpal::traits::HostTrait;
use cpal::Device;

pub fn default_input_device() -> Device {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("no output device available");
    device
}

pub fn get_input_devices() -> String {
    String::from("Test")
}

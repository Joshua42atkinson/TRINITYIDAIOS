use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
fn main() {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no input device");
    println!("Input device: {}", device.name().unwrap());
}

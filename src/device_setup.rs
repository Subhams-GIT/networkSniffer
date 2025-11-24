use crate::start;
use pcap::{Capture, Device};
use std::time::Duration;
pub fn setup_device(time: &mut Duration, interface: String) {
    let all_device = Device::list().unwrap();
    let main_device = all_device
        .iter()
        .find(|device| device.name == interface)
        .unwrap()
        .clone();
    println!("Using device: {:?}", main_device);
    let mut cap = Capture::from_device(main_device)
        .unwrap()
        .promisc(true)
        .snaplen(5000)
        .open()
        .unwrap();
    start::start(time, &mut cap);
}

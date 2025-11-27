// use crate::start;
use pcap::{Active, Capture, Device};
use std::time::Duration;
pub fn setup_device(time: &mut Duration, interface: String)->(Duration,Box<Capture<Active>>) {
    let all_device = Device::list().unwrap();
    let main_device = all_device
        .iter()
        .find(|device| device.name == interface)
        .unwrap()
        .clone();
    
    let cap = Capture::from_device(main_device)
        .unwrap()
        .promisc(true)
        .snaplen(5000)
        .open()
        .unwrap();
   let capture=Box::new(cap); 
    return (*time,capture);
   // let _res= start::start(time, &mut cap);
}
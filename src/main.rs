use networkSniffer::{interface,device_setup};

fn main() {
    // select the interface 
  let (mut time,interface)=interface::get_config();
  // get the device and set it up for packet capturing 
  device_setup::setup_device(&mut time,interface);
}

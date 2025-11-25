use networkSniffer::{device_setup, interface};

fn main() {
    let (mut time, interface) = interface::get_config();
    device_setup::setup_device(&mut time, interface);
    
}

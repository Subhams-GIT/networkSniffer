use networkSniffer::{device_setup, interface,start};

fn main() {
    let (mut time, interface) = interface::get_config();
    let (mut time,capture)=device_setup::setup_device(&mut time, interface);
    start::start(&mut time, *capture).unwrap();
}

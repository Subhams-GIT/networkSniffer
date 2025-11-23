use std::time::{Duration,SystemTime};
use pcap::{Capture, Device};


pub fn setup_device(time:&mut Duration,interface:String) {
 
    let all_device = Device::list().unwrap();
    let main_device=all_device.iter().find(|device| device.name==interface).unwrap().clone();
    println!("Using device: {:?}", main_device);
    let mut cap = Capture::from_device(main_device)
        .unwrap()
        .promisc(true)
        .snaplen(5000)
        .open()
        .unwrap();

    let start = SystemTime::now();
    loop {
        std::thread::sleep(Duration::from_secs(1));

        match start.elapsed() {
            Ok(elapsed) if elapsed > *time => {
                return;
            }
            _ => {
                match cap.next_packet(){
                    Ok(packet)=>{
                        println!("Received packet! ");
                    },
                    Err(e)=>{
                        println!("An error occurred while reading packet: {}", e);
                    }
                }
            },
        }
    }
}

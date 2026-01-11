use etherparse::*;
use pcap_file::pcap::PcapReader;
use std::fs::File;

pub fn parse() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("packets.pcap")?;

    let mut reader = PcapReader::new(file).unwrap();

    while let Some(pkt) = reader.next_packet() {
        let pkt = pkt.unwrap();
        println!("{:?}", pkt);
    }

    Ok(())
}

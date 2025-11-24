use pcap::{Active, Capture, Packet};
use std::fs;
use std::io::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
// construct a pcap file
// 1. global header
// 2. per packet data
// 3.
#[repr(C)] // forces the rust compiler to use the specified layout
struct GlobalHeader {
    magic_number: u32,
    version_major: u16,
    version_minor: u16,
    thiszone: i32,
    sigfigs: i32,
    snaplen: i32,
    network: i32,
}
#[repr(C)]
struct PacketHeader {
    ts_sec: u32,
    ts_usec: u32,
    incl_len: u32,
    orig_len: u32,
}

pub fn start(time: &mut Duration, cap: &mut Capture<Active>) -> std::io::Result<()> {
    let start = SystemTime::now();
    let mut file = fs::File::create("packets.pcap")?;
    let gh = global_header();
    file.write_all(&gh.magic_number.to_le_bytes())?;
    file.write_all(&gh.version_major.to_le_bytes())?;
    file.write_all(&gh.version_minor.to_le_bytes())?;
    file.write_all(&gh.thiszone.to_le_bytes())?;
    file.write_all(&gh.sigfigs.to_le_bytes())?;
    file.write_all(&gh.snaplen.to_le_bytes())?;
    file.write_all(&gh.network.to_le_bytes())?;
    loop {
        std::thread::sleep(Duration::from_secs(1));

        match start.elapsed() {
            Ok(elapsed) if elapsed > *time => {
                return Ok(());
            }
            _ => match cap.next_packet() {
                Ok(packet) => {
                    let ph = packet_header(&packet);
                    file.write_all(&ph.ts_sec.to_le_bytes())?;
                    file.write_all(&ph.ts_usec.to_le_bytes())?;
                    file.write_all(&ph.incl_len.to_le_bytes())?;
                    file.write_all(&ph.orig_len.to_le_bytes())?;
                    file.write_all(&packet.data)?;
                    println!("Received packet! ");
                }
                Err(e) => {
                    println!("An error occurred while reading packet: {}", e);
                }
            },
        }
    }
}

fn global_header() -> GlobalHeader {
    GlobalHeader {
        magic_number: 0xA1B2C3D4,
        version_major: 2,
        version_minor: 4,
        thiszone: 0,
        sigfigs: 0,
        snaplen: 65535,
        network: 1,
    }
}

fn packet_header(packet: &Packet) -> PacketHeader {
    PacketHeader {
        ts_sec: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32,
        ts_usec: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_micros(),
        incl_len: packet.len() as u32,
        orig_len: packet.len() as u32,
    }
}

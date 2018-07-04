extern crate pcap;
extern crate packetspammer;

use std::process::exit;
use std::time::Duration;
use std::thread::sleep;
use std::process::Command;

use packetspammer::*;

use pcap::{Capture, Device};

fn main() {
    let opt = opt::get_args();

    // Get the wireless device to use
    // If the user doesn't specify a device, try the default first
    let device = if opt.interface == "" {
        Device::lookup()
            .unwrap_or_else(|_| { eprintln!("No default device available"); exit(1) })
    } else {
        Device::list()
            // No devices could be listed
            .unwrap_or_else(|_| { eprintln!("No devices available"); exit(1)})
            // Iterate over devices and find the first one with the same name
            .into_iter()
            .filter(|d| d.name == opt.interface)
            .take(1).next()
            .unwrap_or_else(|| { 
                eprintln!("No device \"{}\" available", opt.interface); exit(1) 
            })
    };

    // Open the device and use the same defaults as the `gr-ieee802-11` version
    let mut capture = Capture::from_device(device)
        .unwrap_or_else(|_| { eprintln!("Unable to open wireless device"); exit(1) })
        .snaplen(800)
        .promisc(true)
        .timeout(20)
        .open()
        .unwrap_or_else(|_| {
            eprint!("Unable to open wireless device");
            if let Ok(euid) = Command::new("id").arg("-u").output() {
                if euid.stdout != "0".as_bytes() {
                    eprintln!(". Try running again as root");
                }
            }
            eprint!("");
            exit(1)
        });

    let delay = 1_000_000_000 / opt.rate;

    println!("rate: {}", opt.rate);
    println!("number: {}", opt.number);
    println!("delay: {}", delay);
    println!("size: {}", opt.size);

    // Set up the RNG
    let mut rng = XorShift::default();

    // Create and send each packet
    for number in 0..opt.number {

        let mut buf = vec![];

        // Add the headers
        buf.extend_from_slice(&RADIOTAP_HEADER);
        buf.extend_from_slice(&WIFI_HEADER);
        buf.extend_from_slice(&LLC_HEADER);

        // Add number and size as payload
        buf.extend_from_slice(format!("{:6} {:6} ", number, opt.size).as_bytes());

        // Add the rest of the random data
        // `-14` comes from the above payload info
        for _ in 0..opt.size - 14 {
            buf.push(rng.rand_u8());
        }

        println!("Sending packet: \n{:?}", buf);

        // Send the packet and check for errors
        if let Err(e) = capture.sendpacket(buf) {
            eprintln!("Error injecting packet: {}", e);
            exit(1);
        }

        // Wait to send next packet
        sleep(Duration::new(0, delay as u32));
    }
}

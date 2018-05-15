#[macro_use]
extern crate structopt;
extern crate pcap;
extern crate packetspammer;

use std::process::exit;
use std::time::Duration;
use std::thread::sleep;

use structopt::StructOpt;
use packetspammer::*;

use pcap::{Capture, Device};

#[derive(StructOpt, Debug)]
#[structopt(name = "packetspammer")]
struct Opt {

    /// interface
    #[structopt(name = "interface", default_value = "")]
    interface: String,

    /// number of packets to send
    #[structopt(short = "n", long = "number", default_value = "50")]
    number: usize,

    /// packets per second
    #[structopt(short = "r", long = "rate", default_value = "1")]
    rate: usize,

    /// packet size in bytes (incl MAC header and CRC)
    #[structopt(short = "s", long = "size", default_value = "128")]
    size: usize,
}

fn main() {
    let opt = Opt::from_args();

    // If the user doesn't specify a device, try the default first
    let device = if opt.interface == "" {
        Device::lookup()
            .unwrap_or_else(|_| { eprintln!("No default device available"); exit(1) })
    } else {
        Device::list()
            // No devices could be listed
            .unwrap_or_else(|_| { eprintln!("No devices available"); exit(1)})
            // Iterate over devices and find the one with the same name
            .into_iter()
            .filter(|d| d.name == opt.interface)
            // There should only be one match with the same name; if the list is empty then
            // that means a match wasn't found
            .collect::<Vec<_>>().pop()
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
        .unwrap_or_else(|_| { eprintln!("Unable to open wireless device"); exit(1) });

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

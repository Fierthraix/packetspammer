extern crate packetspammer;
extern crate rand;
extern crate pcap;

use std::process::exit;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

#[macro_use]
extern crate log;

use packetspammer::*;

use pcap::{Capture, Device};
use rand::{thread_rng, Rng};

fn main() {
    // Get Args
    let opt = opt::get_args();

    // Configure Debugging
    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose + 2)
        .init()
        .unwrap();

    // Get the wireless device to use
    // If the user doesn't specify a device, try the default first
    let device = if opt.interface == "" {
        Device::lookup().unwrap_or_else(|_| {
            error!("No default device available");
            exit(1)
        })
    } else {
        Device::list()
            // No devices could be listed
            .unwrap_or_else(|_| {
                warn!("No devices available");
                exit(1)
            })
            // Iterate over devices and find the first one with the same name
            .into_iter()
            .filter(|d| d.name == opt.interface)
            .take(1)
            .next()
            .unwrap_or_else(|| {
                error!("No device \"{}\" available", opt.interface);
                exit(1)
            })
    };

    // Open the device and use the same defaults as the `gr-ieee802-11` version
    let mut capture = Capture::from_device(device)
        .unwrap_or_else(|_| {
            warn!("Unable to open wireless device");
            exit(1)
        })
        .snaplen(800)
        .promisc(true)
        .timeout(20)
        .open()
        .unwrap_or_else(|_| {
            error!("Unable to open wireless device.");

            if let Ok(euid) = Command::new("id").arg("-u").output() {
                if euid.stdout != [b'0'] {
                    info!("Try running the command again as root.");
                }
            }
            exit(1)
        });

    let delay = 1_000_000_000 / opt.rate;

    info!("rate: {}", opt.rate);
    info!("number: {}", opt.number);
    info!("delay: {}", delay);
    info!("size: {}", opt.size);

    // Set up the RNG
    let mut rng = thread_rng();
    let mut mac = DEFAULT_MAC;

    // Create and send each packet
    for number in 0..opt.number {
        let mut buf = vec![];

        // Add the headers
        buf.extend_from_slice(&RADIOTAP_HEADER);
        buf.extend_from_slice(&WIFI_HEADER_START);
        if opt.random_macs {
            rng.fill(&mut mac);
        }
        buf.extend_from_slice(&DEFAULT_MAC);
        buf.extend_from_slice(&WIFI_HEADER_END);
        buf.extend_from_slice(&LLC_HEADER);

        // Add number and size as payload
        buf.extend_from_slice(format!("{:06} {:06} ", number, opt.size).as_bytes());

        // Add the rest of the random data
        // -52 comes from the above payload info
        for _ in 0..opt.size - 52 {
            buf.push(rng.gen::<u8>());
        }

        debug!("Sending packet");
        trace!("{:?}", buf);

        // Send the packet and check for errors
        if let Err(e) = capture.sendpacket(buf) {
            error!("Error injecting packet: {}", e);
            exit(1);
        }

        // Wait to send next packet
        sleep(Duration::new(0, delay as u32));
    }
}

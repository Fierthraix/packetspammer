extern crate structopt;

use structopt::StructOpt;

pub mod opt;

pub const RADIOTAP_HEADER: [u8; 25] = [
    0x00, 0x00, // <-- radiotap version
    0x19, 0x00, // <-- radiotap header length
    0x6f, 0x08, 0x00, 0x00, // <-- bitmap
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // <-- timestamp
    0x00, // <-- flags
    0x0c, // <-- rate
    0x71, 0x09, 0xc0, 0x00, // <-- channel
    0xde, // <-- antsignal
    0x00, // <-- antnoise
    0x01, // <-- antenna
];

pub const DEFAULT_MAC: [u8; 6] = [0x23, 0x23, 0x23, 0x23, 0x23, 0x23];

pub const WIFI_HEADER_START: [u8; 10] =
    [0x88, 0x00, 0x30, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

pub const WIFI_HEADER_END: [u8; 10] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc0, 0x20, 0x20, 0x00];

pub const LLC_HEADER: [u8; 8] = [0xaa, 0xaa, 0x03, 0x00, 0x00, 0x00, 0x88, 0xb5];

#[derive(StructOpt, Debug)]
#[structopt(name = "packetspammer")]
pub struct Opt {
    /// interface
    #[structopt(name = "interface", default_value = "")]
    pub interface: String,

    /// number of packets to send
    #[structopt(short = "n", long = "number", default_value = "50")]
    pub number: usize,

    /// packets per second
    #[structopt(short = "r", long = "rate", default_value = "1")]
    pub rate: usize,

    /// packet size in bytes (incl MAC header and CRC)
    #[structopt(short = "s", long = "size", default_value = "128")]
    pub size: usize,

    /// use a new random MAC address for each packet
    #[structopt(long = "--random-macs")]
    pub random_macs: bool,

    /// logging and output level
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,

    /// silence all output
    #[structopt(short = "q", long = "qiuet")]
    pub quiet: bool,
}

pub fn get_args() -> Opt {
    Opt::from_args()
}

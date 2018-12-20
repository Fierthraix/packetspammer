use structopt::StructOpt;

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

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

}

pub fn get_args() -> Opt {
    Opt::from_args()
}

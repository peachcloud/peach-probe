extern crate env_logger;
#[macro_use]
extern crate log;

mod error;
use crate::error::PeachProbeParseError;

use structopt::StructOpt;
use std::str::FromStr;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "peach-probe",
    about = "a CLI tool for contract testing of the public API's exposed by PeachCloud microservices"
)]
struct Opt {
    /// switch on verbosity
    #[structopt(short, long)]
    verbose: bool,
    // optional list of microservices to filter down to
    services: Vec<Command>
}

#[derive(StructOpt, Debug)]
enum Command {
    Oled,
    Network,
    Stats,
    Menu
}

impl FromStr for Command {
    type Err = PeachProbeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "oled" => Ok(Command::Oled),
            "network" => Ok(Command::Network),
            "stats" => Ok(Command::Stats),
            "menu" => Ok(Command::Menu),
            // due to lifetime questions, wasn't sure how to include the &str in the error
            _ => Err(PeachProbeParseError::InvalidMicroservice{ arg: s.to_string()})
        }
    }
}


fn main() {

    // initialize the logger
    env_logger::init();

    // hello
    info!("Hello, world, its peach probe.");

    // parse cli arguments
    let opt = Opt::from_args();

    // debugging what was parsed
    info!("services: {:?}", opt.services);
    for x in &opt.services {
       info!("service: {:?}", x);
    }
    if opt.verbose {
        info!("using verbose mode")
    }

}

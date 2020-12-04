use log::info;
use std::str::FromStr;
use const_format::formatcp;

use structopt::StructOpt;

mod error;
mod probe;
use crate::error::PeachProbeParseError;
use crate::probe::PeachProbe;


// list of microservices allowed as arguments
pub const POSSIBLE_MICROSERVICE_ARGS:&str = "menu, network, oled, stats";

#[derive(StructOpt, Debug)]
#[structopt(
    name = "peach-probe",
    about = "a CLI tool for contract testing of the public API's exposed by PeachCloud microservices"
)]
pub struct Opt {
    /// switch on verbosity
    #[structopt(short, long)]
    verbose: bool,
    #[structopt(help = formatcp!("an optional list of microservices to probe [possible values: {}]", POSSIBLE_MICROSERVICE_ARGS))]
    services: Vec<Microservice>,
}

#[derive(StructOpt, Debug)]
pub enum Microservice {
    Oled,
    Network,
    Stats,
    Menu,
}

impl FromStr for Microservice {
    type Err = PeachProbeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "oled" => Ok(Microservice::Oled),
            "network" => Ok(Microservice::Network),
            "stats" => Ok(Microservice::Stats),
            "menu" => Ok(Microservice::Menu),
            _ => Err(PeachProbeParseError::InvalidMicroservice { arg: s.to_string() }),
        }
    }
}


fn main() {
    // initialize the logger
    env_logger::init();

    // parse cli arguments
    let opt = Opt::from_args();

    // debugging what was parsed
    info!("services: {:?}", opt.services);
    if opt.verbose {
        info!("using verbose mode")
    }

    let services;
    // if not arguments were provided, then we probe all services
    if opt.services.is_empty() {
        services = vec![
            Microservice::Network,
            Microservice::Oled,
            Microservice::Stats,
        ]
    } else {
        services = opt.services;
    }

    // instantiate the probe
    let mut peach_probe: PeachProbe = PeachProbe::new();

    // iterate through services and run probe tests on them
    for service in services {
        match service {
            Microservice::Stats => {
                //stats_probe::probe_stats();
                peach_probe.stats();
            }
            _ => info!("probe for service {:?} not yet implemented", service),
        }
    }

    // reporting
    println!("[ generating probe report ]");
    for result in peach_probe.results {
        println!("{}", result.microservice);
        // success
        println!("{} successful endpoint calls", result.success);
        // failure
        println!("{} failed endpoint calls", result.failure);
    }
}

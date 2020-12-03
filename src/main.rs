mod error;
mod probe;

use std::str::FromStr;

use log::info;
use structopt::StructOpt;

use crate::error::PeachProbeParseError;
use crate::probe::PeachProbe;

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
    services: Vec<Microservice>,
}

#[derive(StructOpt, Debug)]
enum Microservice {
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
            // due to lifetime questions, wasn't sure how to include the &str in the error
            _ => Err(PeachProbeParseError::InvalidMicroservice { arg: s.to_string() }),
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
        info!("probing service: {:?}", service);
        match service {
            Microservice::Stats => {
                //stats_probe::probe_stats();
                peach_probe.stats();
            }
            _ => info!("probe for service {:?} not yet implemented", service),
        }
    }

    // reporting
    for result in peach_probe.results {
        println!("{}", result.microservice);
        // success
        println!("{} successful endpoint calls", result.success);
        // failure
        println!("{} failed endpoint calls", result.failure);
    }
}

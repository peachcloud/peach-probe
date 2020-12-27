use log::info;

use structopt::StructOpt;
use clap::arg_enum;

mod probe;
mod vars;

use crate::probe::PeachProbe;


#[derive(StructOpt, Debug)]
 #[structopt(
     name = "peach-probe",
     about = "a CLI tool for contract testing of the public API's exposed by PeachCloud microservices"
 )]
 struct Opt {
     #[structopt(short, long)]
     verbose: bool,
     #[structopt(possible_values = &Microservice::variants(), case_insensitive = true)]
     services: Vec<Microservice>,
 }

 arg_enum! {
     #[derive(Debug)]
     enum Microservice {
         PeachOled,
         PeachNetwork,
         PeachStats,
         PeachMenu,
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
            Microservice::PeachNetwork,
            Microservice::PeachOled,
            Microservice::PeachStats,
        ]
    } else {
        services = opt.services;
    }

    // instantiate the probe
    let mut probe: PeachProbe = PeachProbe::new();

    // iterate through services and run probe tests on them
    for service in services {
        match service {
            Microservice::PeachStats => {
                probe.peach_stats();
            },
            Microservice::PeachOled => {
                probe.peach_oled();
            },
            Microservice::PeachNetwork => {
                probe.peach_network();
            },
            _ => info!("probe for service {:?} not yet implemented", service),
        }
    }

    // final report of how many microservices returned successes and failures
    println!("[ generating report ]");
    for result in probe.results {
        let num_failures = result.failures.len();
        let report;
        if num_failures == 0 {
            report = format!("++ {} microservice is online with all endpoints running: {:?}", result.microservice, result.successes);
            println!("{}", report);
        } else {
            report = format!("++ {} microservice had {} endpoints that returned errors: {:?}", result.microservice, num_failures, result.failures);
            eprintln!("{}", report);
        }
    }
}

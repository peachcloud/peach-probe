use peach_lib::error::StatsError;
use peach_lib::stats_client;

pub struct ProbeResult {
    pub microservice: String,
    pub success: i32,
    pub failure: i32,
}

impl ProbeResult {
    fn new(microservice: String) -> ProbeResult {
        ProbeResult {
            microservice,
            success: 0,
            failure: 0,
        }
    }
}

pub struct PeachProbe {
    pub results: Vec<ProbeResult>,
}

impl PeachProbe {
    pub fn new() -> PeachProbe {
        PeachProbe {
            results: Vec::new(),
        }
    }

    pub fn stats(&mut self) {
        println!("[ probing stats microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("stats".to_string());

        // cpu_stats_percent
        match stats_client::cpu_stats_percent() {
            Ok(_) => {
                println!("** cpu_stats_percent endpoint is online");
                result.success += 1;
            }
            Err(e) => {
                eprintln!("** cpu_stats_percent is offline");
                match e {
                    StatsError::StatsHttp(e) => eprintln!("{:?}", e.description()),
                    StatsError::StatsClient(e) => eprintln!("{:?}", e.description()),
                    _ => (),
                }
                result.failure += 1;
            }
        }

        // cpu_stats_percent
        match stats_client::disk_usage() {
            Ok(_) => {
                println!("** disk_usage endpoint is online");
                result.success += 1;
            }
            Err(e) => {
                eprintln!("** disk_usage endpoint is offline");
                match e {
                    StatsError::StatsHttp(e) => eprintln!("{:?}", e.description()),
                    StatsError::StatsClient(e) => eprintln!("{:?}", e.description()),
                    _ => (),
                }
                result.failure += 1;
            }
        }

        self.results.push(result)
    }
}

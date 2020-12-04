use peach_lib::error::StatsError;
use peach_lib::stats_client;

pub struct ProbeResult {
    pub microservice: String,
    // vector of names of endpoints which had errors
    pub failures: Vec<String>,
    // vector of names of endpoints which returned successfully
    pub successes: Vec<String>
}

impl ProbeResult {
    fn new(microservice: String) -> ProbeResult {
        ProbeResult {
            microservice,
            failures: Vec::new(),
            successes: Vec::new()
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

    fn probe_endpoint(endpoint_fn: fn() -> Result<stats_client::CpuStatPercentages, StatsError>, endpoint_name: &str, result: &mut ProbeResult) {
        match endpoint_fn() {
            Ok(_) => {
                println!("++ {} endpoint is online", endpoint_name);
                result.successes.push(endpoint_name.to_string());
            }
            Err(e) => {
                eprintln!("++ {} endpoint is offline", endpoint_name);
                match e {
                    StatsError::StatsHttp(e) => eprintln!("Returned error: {:?}\n", e.description()),
                    StatsError::StatsClient(e) => eprintln!("Returned error: {:?}\n", e.description()),
                    _ => (),
                }
                result.failures.push(endpoint_name.to_string());
            }
        }
    }


    pub fn stats(&mut self) {
        println!("[ probing stats microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("stats".to_string());

        // probe endpoints
        PeachProbe::probe_endpoint(stats_client::cpu_stats_percent, "cpu_stats_percent", &mut result);

        // cpu_stats_percent
        match stats_client::cpu_stats_percent() {
            Ok(_) => {
                println!("++ cpu_stats_percent endpoint is online");
                result.successes.push("cpu_stats_percent".to_string());
            }
            Err(e) => {
                eprintln!("++ cpu_stats_percent is offline");
                match e {
                    StatsError::StatsHttp(e) => eprintln!("Returned error: {:?}\n", e.description()),
                    StatsError::StatsClient(e) => eprintln!("Returned error: {:?}\n", e.description()),
                    _ => (),
                }
                result.failures.push("cpu_stats_percent".to_string());
            }
        }

        // cpu_stats_percent
        match stats_client::load_average() {
            Ok(_) => {
                println!("++ disk_usage endpoint is online");
                result.successes.push("disk_usage".to_string());
            }
            Err(e) => {
                eprintln!("++ disk_usage endpoint is offline");
                match e {
                    StatsError::StatsHttp(e) => eprintln!("Returned error: {:?}\n", e.description()),
                    StatsError::StatsClient(e) => eprintln!("Returned error: {:?}\n", e.description()),
                    _ => (),
                }
                result.failures.push("disk_usage".to_string());
            }
        }

        // load_average
        match stats_client::load_average() {
            Ok(_) => {
                println!("++ load_average endpoint is online");
                result.successes.push("load_average".to_string());
            }
            Err(e) => {
                eprintln!("++ load_average endpoint is offline");
                match e {
                    StatsError::StatsHttp(e) => eprintln!("Returned error: {:?}\n", e.description()),
                    StatsError::StatsClient(e) => eprintln!("Returned error: {:?}\n", e.description()),
                    _ => (),
                }
                result.failures.push("load_average".to_string());
            }
        }

        self.results.push(result)
    }
}

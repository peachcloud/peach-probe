use peach_lib::error::StatsError;
use peach_lib::stats_client;
use peach_lib::oled_client;
use peach_lib::error::OledError;

use crate::vars::PEACH_LOGO;

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

    /// helper function for probing an endpoint on the stats microfunction and collecting errors for a final report
    fn probe_stats_endpoint<T>(endpoint_result: Result<T, StatsError>, endpoint_name: &str, result: &mut ProbeResult) {
        match endpoint_result {
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

    /// probes all endpoints on the stats microservice
    pub fn stats(&mut self) {
        println!("[ probing stats microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("stats".to_string());

        // probe endpoints
        PeachProbe::probe_stats_endpoint(stats_client::cpu_stats_percent(), "cpu_stats_percent", &mut result);
        PeachProbe::probe_stats_endpoint(stats_client::load_average(), "load_average", &mut result);
        PeachProbe::probe_stats_endpoint(stats_client::disk_usage(), "disk_usage", &mut result);
        PeachProbe::probe_stats_endpoint(stats_client::mem_stats(), "mem_stats", &mut result);
        PeachProbe::probe_stats_endpoint(stats_client::ping(), "ping", &mut result);
        PeachProbe::probe_stats_endpoint(stats_client::uptime(), "uptime", &mut result);

        // save result
        self.results.push(result)
    }

    /// helper function for probing an endpoint on the oled microfunction and collecting errors for a final report
    fn probe_oled_endpoint<T>(endpoint_result: Result<T, OledError>, endpoint_name: &str, result: &mut ProbeResult) {
        match endpoint_result {
            Ok(_) => {
                println!("++ {} endpoint is online", endpoint_name);
                result.successes.push(endpoint_name.to_string());
            }
            Err(e) => {
                eprintln!("++ {} endpoint is offline", endpoint_name);
                match e {
                    OledError::OledHttp(e) => eprintln!("Returned error: {:?}\n", e.description()),
                    OledError::OledClient(e) => eprintln!("Returned error: {:?}\n", e.description()),
                    _ => (),
                }
                result.failures.push(endpoint_name.to_string());
            }
        }
    }

    /// probes all endpoints on the oled microservice
    pub fn oled(&mut self) {
        println!("[ probing oled microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("oled".to_string());

        // probe endpoints
        PeachProbe::probe_oled_endpoint(oled_client::clear(), "clear", &mut result);
        PeachProbe::probe_oled_endpoint(oled_client::ping(), "ping", &mut result);
        PeachProbe::probe_oled_endpoint(oled_client::write(0, 0, "Running peach-probe", "6x8"), "write", &mut result);
        PeachProbe::probe_oled_endpoint(oled_client::flush(), "flush", &mut result);

        // probe draw endpoint
        let bytes = PEACH_LOGO.to_vec();
        PeachProbe::probe_oled_endpoint(
            oled_client::draw(bytes, 64, 64, 32, 10),
            "draw", &mut result);
        oled_client::flush();

        // test power off endpoint
         PeachProbe::probe_oled_endpoint(oled_client::power(false), "power-off", &mut result);
         PeachProbe::probe_oled_endpoint(oled_client::power(true), "power-on", &mut result);

        // save result
        self.results.push(result)
    }
}

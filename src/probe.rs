use peach_lib::stats_client;
use peach_lib::oled_client;
use peach_lib::network_client;
use peach_lib::error::PeachError;

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

    /// helper function for probing an endpoint on a peach microservice and collecting errors for a final report
    fn probe_peach_endpoint<T>(endpoint_result: Result<T, PeachError>, endpoint_name: &str, result: &mut ProbeResult) {
        match endpoint_result {
            Ok(_) => {
                println!("++ {} endpoint is online", endpoint_name);
                result.successes.push(endpoint_name.to_string());
            }
            Err(e) => {
                eprintln!("++ {} endpoint is offline", endpoint_name);
                match e {
                    PeachError::JsonRpcHttp(e) => eprintln!("Returned JsonRpcHTTP error: {:#?}\n", e),
                    PeachError::JsonRpcCore(e) => eprintln!("Returned JsonRpcCore error: {:#?}\n", e),
                    // QUESTION: PeachError::Serde does not implement .description -- should we show a message in another way?
                    PeachError::Serde(_) => eprintln!("Returned Serde Json serialization error\n")
                }
                result.failures.push(endpoint_name.to_string());
            }
        }
    }

    /// probes all endpoints on the peach-stats microservice
    pub fn peach_stats(&mut self) {
        println!("[ probing peach-stats microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("peach-stats".to_string());

        // probe endpoints
        PeachProbe::probe_peach_endpoint(stats_client::cpu_stats_percent(), "cpu_stats_percent", &mut result);
        PeachProbe::probe_peach_endpoint(stats_client::load_average(), "load_average", &mut result);
        PeachProbe::probe_peach_endpoint(stats_client::disk_usage(), "disk_usage", &mut result);
        PeachProbe::probe_peach_endpoint(stats_client::mem_stats(), "mem_stats", &mut result);
        PeachProbe::probe_peach_endpoint(stats_client::ping(), "ping", &mut result);
        PeachProbe::probe_peach_endpoint(stats_client::uptime(), "uptime", &mut result);

        // save result
        self.results.push(result)
    }

    /// probes all endpoints on peach-network microservice
    pub fn peach_network(&mut self) {
        println!("[ probing peach-network microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("peach-network".to_string());

        // probe endpoints
        PeachProbe::probe_peach_endpoint(network_client::activate_ap(), "activate_ap", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::activate_client(), "activate_client", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::add("peach-probe-test-ssid", "peach-probe-test-pass"), "add", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::available_networks("wlan0"), "available_networks", &mut result);

        PeachProbe::probe_peach_endpoint(network_client::connect("peach-probe-test-ssid", "wlan0"), "connect", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::id("wlan0", "peach-probe-test-ssid"), "id", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::ip("wlan0"), "ip", &mut result);
//        PeachProbe::probe_peach_endpoint(network_client::modify("peach-probe-test-ssid", "wlan0", "test-pass2"), "modify", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::ping(), "ping", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::reconfigure(), "reconfigure", &mut result);
//        PeachProbe::probe_peach_endpoint(network_client::reconnect(), "reconnect", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::rssi("wlan0"), "rssi", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::rssi_percent("wlan0"), "rssi-percent", &mut result);
//        PeachProbe::probe_peach_endpoint(network_client::disable("peach-probe-test-ssid", "wlan0"), "disable", &mut result);
//        PeachProbe::probe_peach_endpoint(network_client::disconnect("wlan0"), "disconnect", &mut result);
//        PeachProbe::probe_peach_endpoint(network_client::delete("peach-probe-test-ssid", "wlan0"), "delete", &mut result);
//        PeachProbe::probe_peach_endpoint(network_client::save(), "save", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::saved_networks(), "saved_networks", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::ssid("wlan0"), "ssid", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::state("wlan0"), "state", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::status("wlan0"), "status", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::traffic("wlan0"), "traffic", &mut result);

        // save result
        self.results.push(result)
    }

    /// probes all endpoints on the peach-oled microservice
    pub fn peach_oled(&mut self) {
        println!("[ probing peach-oled microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("peach-oled".to_string());

        // probe endpoints
        PeachProbe::probe_peach_endpoint(oled_client::ping(), "ping", &mut result);
        PeachProbe::probe_peach_endpoint(oled_client::write(0, 0, "Peach-probe display", "6x8"), "write", &mut result);

        // probe draw endpoint
        let bytes = PEACH_LOGO.to_vec();
        PeachProbe::probe_peach_endpoint(
            oled_client::draw(bytes, 64, 64, 32, 10),
            "draw", &mut result);

        // probe clear and flush after draw and write (so that there are no visual artifacts from peach-probe on the oled display)
//        PeachProbe::probe_peach_endpoint(oled_client::clear(), "clear", &mut result);
        PeachProbe::probe_peach_endpoint(oled_client::flush(), "flush", &mut result);

        // test power off endpoint
         PeachProbe::probe_peach_endpoint(oled_client::power(false), "power-off", &mut result);
         PeachProbe::probe_peach_endpoint(oled_client::power(true), "power-on", &mut result);

        // save result
        self.results.push(result)
    }
}

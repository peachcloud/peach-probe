use peach_lib::error::PeachError;
use peach_lib::network_client;
use peach_lib::oled_client;
use peach_lib::stats_client;

use crate::vars::PEACH_LOGO;

// this struct stores the results of probing a particular microservice
pub struct ProbeResult {
    // string of the name of the service
    pub microservice: String,
    // string of the version of this service currently installed
    pub version: String,
    // vector of names of endpoints which had errors
    pub failures: Vec<String>,
    // vector of names of endpoints which returned successfully
    pub successes: Vec<String>,
    // bool which returns true if systemctl status says service is running
    pub is_running: bool,
}

impl ProbeResult {
    fn new(microservice: String) -> ProbeResult {
        ProbeResult {
            microservice,
            failures: Vec::new(),
            successes: Vec::new(),
            is_running: false,
            version: "".to_string(),
        }
    }
}

// this struct implements probes for all microservices and data structures
// for storing the results of all probes
pub struct PeachProbe {
    pub results: Vec<ProbeResult>,
    pub verbose: bool,
}

impl PeachProbe {
    pub fn new(verbose: bool) -> PeachProbe {
        PeachProbe {
            results: Vec::new(),
            verbose,
        }
    }


    // helper function which gets the version of the microservice running using apt-get
    fn get_service_version(service: &str) -> String {
        let output = std::process::Command::new("/usr/bin/apt")
        .arg("list")
        .arg(service)
        .output()
        .expect("failed");
        // TODO: use a regex here to just get the version number from this string
        let command_output = std::str::from_utf8(&output.stdout).unwrap().to_string();
        // use a regex to get the version number from the string
        let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
        for cap in re.captures_iter(command_output) {
            println!("cap1: {}, cap2: {}", &cap[1], &cap[2]);
        }
        "version-xyz"
    }

    /// helper function for probing an endpoint on a peach microservice and collecting errors for a final report
    fn probe_peach_endpoint<T>(
        &mut self,
        endpoint_result: Result<T, PeachError>,
        endpoint_name: &str,
        result: &mut ProbeResult,
    ) {
        match endpoint_result {
            Ok(_) => {
                if self.verbose {
                    println!("++ {} endpoint is online", endpoint_name);
                }
                result.successes.push(endpoint_name.to_string());
            }
            Err(e) => {
                eprintln!("++ {} endpoint is offline", endpoint_name);
                match e {
                    PeachError::JsonRpcHttp(e) => {
                        eprintln!("Returned JsonRpcHTTP error: {:#?}\n", e)
                    }
                    PeachError::JsonRpcCore(e) => {
                        eprintln!("Returned JsonRpcCore error: {:#?}\n", e)
                    }
                    // QUESTION: PeachError::Serde does not implement .description -- should we show a message in another way?
                    PeachError::Serde(_) => eprintln!("Returned Serde Json serialization error\n"),
                }
                result.failures.push(endpoint_name.to_string());
            }
        }
    }

    /// helper function for probing an endpoint on a peach microservice which expects a particular JsonRPCCore Error
    fn probe_assert_error_endpoint<T>(
        &mut self,
        endpoint_result: Result<T, PeachError>,
        endpoint_name: &str,
        expected_error_code: i64,
        result: &mut ProbeResult,
    ) {
        match endpoint_result {
            Ok(_) => {
                eprintln!("++ this endpoint should not return successfully during peach-probe, something is strange");
                result.failures.push(endpoint_name.to_string());
            }
            Err(e) => {
                match e {
                    PeachError::JsonRpcCore(e) => {
                        match e.kind() {
                            // this is the expected error, all other errors are unexpected
                            jsonrpc_client_core::ErrorKind::JsonRpcError(err) => {
                                if err.code.code() == expected_error_code {
                                    if self.verbose {
                                        println!("++ endpoint is online");
                                    }
                                    result.successes.push(endpoint_name.to_string());
                                } else {
                                    eprintln!("++ {} endpoint is offline", endpoint_name);
                                    eprintln!("Returned JsonRpcCore error with unexpected code or message: {:#?}\n", e);
                                    result.failures.push(endpoint_name.to_string());
                                }
                            }
                            _ => {
                                eprintln!("++ {} endpoint is offline", endpoint_name);
                                eprintln!("Returned unexpected JsonRpcCore error: {:#?}\n", e);
                                result.failures.push(endpoint_name.to_string());
                            }
                        }
                    }
                    PeachError::JsonRpcHttp(e) => {
                        eprintln!("++ {} endpoint is offline", endpoint_name);
                        eprintln!("Returned JsonRpcHTTP error: {:#?}\n", e);
                        result.failures.push(endpoint_name.to_string());
                    }
                    PeachError::Serde(_) => {
                        eprintln!("++ {} endpoint is offline", endpoint_name);
                        eprintln!("Returned Serde Json serialization error\n");
                        result.failures.push(endpoint_name.to_string());
                    }
                }
            }
        }
    }

    /// probes all endpoints on the peach-stats microservice
    pub fn peach_stats(&mut self) {
        println!("[ probing peach-stats microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("peach-stats".to_string());

        // get version of service

        // probe endpoints
        self.probe_peach_endpoint(
            stats_client::cpu_stats_percent(),
            "cpu_stats_percent",
            &mut result,
        );
        self.probe_peach_endpoint(stats_client::load_average(), "load_average", &mut result);
        self.probe_peach_endpoint(stats_client::disk_usage(), "disk_usage", &mut result);
        self.probe_peach_endpoint(stats_client::mem_stats(), "mem_stats", &mut result);
        self.probe_peach_endpoint(stats_client::ping(), "ping", &mut result);
        self.probe_peach_endpoint(stats_client::uptime(), "uptime", &mut result);

        // save result
        self.results.push(result)
    }

    /// probes all endpoints on peach-network microservice
    pub fn peach_network(&mut self) {
        println!("[ probing peach-network microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("peach-network".to_string());

        // probe endpoints which should successfully return if online
        self.probe_peach_endpoint(network_client::activate_ap(), "activate_ap", &mut result);
        self.probe_peach_endpoint(
            network_client::activate_client(),
            "activate_client",
            &mut result,
        );
        self.probe_peach_endpoint(
            network_client::add("peach-probe-test-ssid", "peach-probe-test-pass"),
            "add",
            &mut result,
        );
        self.probe_peach_endpoint(
            network_client::available_networks("wlan0"),
            "available_networks",
            &mut result,
        );
        self.probe_peach_endpoint(
            network_client::id("wlan0", "peach-probe-test-ssid"),
            "id",
            &mut result,
        );
        self.probe_peach_endpoint(network_client::ip("wlan0"), "ip", &mut result);
        self.probe_peach_endpoint(network_client::ssid("wlan0"), "ssid", &mut result);
        self.probe_peach_endpoint(network_client::ping(), "ping", &mut result);
        self.probe_peach_endpoint(network_client::reconfigure(), "reconfigure", &mut result);
        self.probe_peach_endpoint(
            network_client::saved_networks(),
            "saved_networks",
            &mut result,
        );
        self.probe_peach_endpoint(network_client::state("wlan0"), "state", &mut result);
        self.probe_peach_endpoint(network_client::traffic("wlan0"), "traffic", &mut result);

        // if online, the following functions should return an error which we should catch and confirm
        self.probe_assert_error_endpoint(
            network_client::connect("peach-probe-test-ssid", "wlan0"),
            "connect",
            -32027,
            &mut result,
        );
        // change this to be confirm the correct error code
        self.probe_assert_error_endpoint(
            network_client::disable("peach-probe-test-ssid", "wlan0"),
            "disable",
            -32013,
            &mut result,
        );

        // the following functions should return an error which we should catch and confirm,
        // but waiting for PR to peach-network to provide more verbose error messages for these endpoints
        //        self.probe_peach_endpoint(network_client::status("wlan0"), "status", &mut result);
        //        self.probe_peach_endpoint(network_client::rssi("wlan0"), "rssi", &mut result);
        //        self.probe_peach_endpoint(network_client::rssi_percent("wlan0"), "rssi-percent", &mut result);

        // save result
        self.results.push(result)
    }

    /// probes all endpoints on the peach-oled microservice
    pub fn peach_oled(&mut self) {
        println!("[ probing peach-oled microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("peach-oled".to_string());

        // get current installed version of service using apt-get
        result.version = PeachProbe::get_service_version("peach-oled").to_string();

        // probe endpoints
        self.probe_peach_endpoint(oled_client::ping(), "ping", &mut result);
        self.probe_peach_endpoint(
            oled_client::write(0, 0, "Peach-probe display", "6x8"),
            "write",
            &mut result,
        );

        // probe draw endpoint
        let bytes = PEACH_LOGO.to_vec();
        self.probe_peach_endpoint(
            oled_client::draw(bytes, 64, 64, 32, 10),
            "draw",
            &mut result,
        );

        // probe clear and flush after draw and write (so that there are no visual artifacts from peach-probe on the oled display)
        //        self.probe_peach_endpoint(oled_client::clear(), "clear", &mut result);
        self.probe_peach_endpoint(oled_client::flush(), "flush", &mut result);

        // test power off endpoint
        self.probe_peach_endpoint(oled_client::power(false), "power-off", &mut result);
        self.probe_peach_endpoint(oled_client::power(true), "power-on", &mut result);

        // save result
        self.results.push(result)
    }
}

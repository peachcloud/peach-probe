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
                    PeachError::JsonRpcHttp(e) => eprintln!("Returned JsonRpcHTTP error: {:?}\n", e),
                    PeachError::JsonRpcCore(e) => eprintln!("Returned JsonRpcCore error: {:?}\n", e),
                    // QUESTION: PeachError::Serde does not implement .description -- should we show a message in another way?
                    PeachError::Serde(_) => eprintln!("Returned Serde Json error\n")
                }
                result.failures.push(endpoint_name.to_string());
            }
        }
    }

    /// probes all endpoints on the peach-stats microservice
    pub fn stats(&mut self) {
        println!("[ probing peach-stats microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("stats".to_string());

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
    pub fn network(&mut self) {
        println!("[ probing peach-network microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("stats".to_string());

        // probe endpoints
        PeachProbe::probe_peach_endpoint(network_client::activate_ap(), "activate_ap", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::activate_client(), "activate_client", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::add("peach-probe-test-ssid", "peach-probe-test-pass"), "add", &mut result);
        PeachProbe::probe_peach_endpoint(network_client::available_networks("wlan0"), "available_networks", &mut result);

        // these comments left here so I remember what else to probe, once above endpoints have been debugged
//    /// JSON-RPC request to list all networks in range of the given interface.
//    pub fn available_networks(&mut self, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to connect the network for the given interface and ID.
//    pub fn connect(&mut self, id: &str, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to delete the credentials for the given network from the wpa_supplicant config.
//    pub fn delete(&mut self, id: &str, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to disable the network for the given interface and ID.
//    pub fn disable(&mut self, id: &str, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to disconnect the network for the given interface.
//    //pub fn disconnect(&mut self, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to get the ID for the given interface and SSID.
//    pub fn id(&mut self, iface: &str, ssid: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to get the IP address for the given interface.
//    pub fn ip(&mut self, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to set a new network password for the given interface and ID.
//    //pub fn modify(&mut self, id: &str, iface: &str, pass: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to check peach-network availability.
//    pub fn ping(&mut self) -> RpcRequest<String>;
//
//    /// JSON-RPC request to reread the wpa_supplicant config for the given interface.
//    pub fn reconfigure(&mut self) -> RpcRequest<String>;
//
//    /// JSON-RPC request to reconnect WiFi for the given interface.
//    //pub fn reconnect(&mut self, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to get the average signal strength (dBm) for the given interface.
//    pub fn rssi(&mut self, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to get the average signal quality (%) for the given interface.
//    pub fn rssi_percent(&mut self, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to save network configuration updates to file.
//    pub fn save(&mut self) -> RpcRequest<String>;
//
//    /// JSON-RPC request to list all networks saved in `wpa_supplicant.conf`.
//    pub fn saved_networks(&mut self) -> RpcRequest<String>;
//
//    /// JSON-RPC request to get the SSID of the currently-connected network for the given interface.
//    pub fn ssid(&mut self, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to get the state for the given interface.
//    pub fn state(&mut self, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to get the status of the given interface.
//    pub fn status(&mut self, iface: &str) -> RpcRequest<String>;
//
//    /// JSON-RPC request to get the network traffic for the given interface.
//    pub fn traffic(&mut self, iface: &str) -> RpcRequest<String>;

        // save result
        self.results.push(result)
    }

    /// probes all endpoints on the peach-oled microservice
    pub fn oled(&mut self) {
        println!("[ probing peach-oled microservice ]");

        // instantiate ProbeResult
        let mut result = ProbeResult::new("oled".to_string());

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

//! Perform JSON-RPC calls to the `peach-stats` microservice.
//!
//! This module contains a JSON-RPC client and associated data structures for
//! making calls to the `peach-stats` microservice. Each RPC has a corresponding
//! method which creates an HTTP transport, makes the call to the RPC
//! microservice and returns the response to the caller. These convenience
//! methods simplify the process of performing RPC calls from other modules.

extern crate jsonrpc_client_http;

use std::env;

use jsonrpc_client_http::HttpTransport;

use crate::error::StatsError;

#[derive(Debug, Deserialize, Serialize)]
pub struct CpuStatPercentages {
    pub user: f32,
    pub system: f32,
    pub idle: f32,
    pub nice: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiskUsage {
    pub filesystem: Option<String>,
    pub one_k_blocks: u64,
    pub one_k_blocks_used: u64,
    pub one_k_blocks_free: u64,
    pub used_percentage: u32,
    pub mountpoint: String,
}


/// Creates a JSON-RPC client with http transport and calls the `peach-stats`
/// `cpu_stats_percent` method.
pub fn cpu_stats_percent() -> std::result::Result<CpuStatPercentages, StatsError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.cpu_stats_percent().call()?;
    let c: CpuStatPercentages = serde_json::from_str(&response)?;

    Ok(c)
}


/// Creates a JSON-RPC client with http transport and calls the `peach-stats`
/// `disk_usage` method.
pub fn disk_usage() -> std::result::Result<String, StatsError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.disk_usage().call()?;

    Ok(response)
}


jsonrpc_client!(pub struct PeachStatsClient {
    /// JSON-RPC request to get measurement of current CPU statistics.
    pub fn cpu_stats_percent(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to get measurement of current disk usage statistics.
    pub fn disk_usage(&mut self) -> RpcRequest<String>;
});

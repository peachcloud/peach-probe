//! Basic error handling

use std::error;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum PeachProbeParseError {
    #[snafu(display("The argument {} is not one of the microservice options for peach-probe", arg))]
    InvalidMicroservice {arg: String}
}


// errors for peach-stats
#[derive(Debug)]
pub enum StatsError {
    StatsHttp(jsonrpc_client_http::Error),
    StatsClient(jsonrpc_client_core::Error),
    StatsSerde(serde_json::error::Error),
}

impl From<jsonrpc_client_http::Error> for StatsError {
    fn from(err: jsonrpc_client_http::Error) -> StatsError {
        StatsError::StatsHttp(err)
    }
}

impl From<jsonrpc_client_core::Error> for StatsError {
    fn from(err: jsonrpc_client_core::Error) -> StatsError {
        StatsError::StatsClient(err)
    }
}

impl From<serde_json::error::Error> for StatsError {
    fn from(err: serde_json::error::Error) -> StatsError {
        StatsError::StatsSerde(err)
    }
}
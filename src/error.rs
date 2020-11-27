//! Basic error handling

use std::error;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum PeachProbeParseError {
    #[snafu(display("The argument {} is not one of the microservice options for peach-probe", arg))]
    InvalidMicroservice {arg: String}
}


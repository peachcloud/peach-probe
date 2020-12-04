use snafu::Snafu;

use crate::POSSIBLE_MICROSERVICE_ARGS;

#[derive(Debug, Snafu)]
pub enum PeachProbeParseError {
    #[snafu(display(
        "'{}' is not one of the microservice options for peach-probe. \
         [possible values: {}]",
        arg, POSSIBLE_MICROSERVICE_ARGS
    ))]
    InvalidMicroservice { arg: String },
}

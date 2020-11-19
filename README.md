# peach-probe

![Generic badge](https://img.shields.io/badge/version-0.1.0-<COLOR>.svg)

Probe PeachCloud microservices to evaluate their state and ensure correct API responses.

`peach-probe` is a CLI tool for contract testing of the public API's exposed by PeachCloud microservices. It is composed of JSON-RPC clients which make calls to the methods of their respective servers and report back on the results.

This utility is intended to provide a rapid means of testing a deployed PeachCloud system and allow informed trouble-shooting in the case of errors.

## Design Specification

Functionality:

 - Probe all microservices
 - Probe a single microservice

Configurability:

 - Microservice address and port
   - Defaults defined in code
   - `port` flag allows the server port of each service to be defined manually
   - Optional: define parameters in a toml file

Reporting:

 - Granular reporting based on flags and options
   - Simple pass / fail
     - Could be similar to systemd: active, degraded, inactive
   - Details on unexpected responses (degraded state)
     - Example: permission denied reponse for `get_status` call for `wlan0` to `peach-network` microservice
     - Possible fixes for errors

## Suggested Implementation

 - Use [structopt](https://crates.io/crates/structopt) for CLI definitions and argument parsing
 - Create a separate module for each microservice's JSON-RPC client (ie. `stats.rs`, `oled.rs` etc.)
   - Use [jsonrpc-client-http](https://crates.io/crates/jsonrpc-client-http)
   - Examples in [peach-web](https://github.com/peachcloud/peach-web) (eg. `src/stats_client.rs`)

## Licensing

AGPL-3.0

use log::{info, warn};
use peach_lib::stats_client;

pub fn probe_stats() {
    info!("++ probing stats microservice");

    // cpu_stats_percent
    let cpu_stats_percent = stats_client::cpu_stats_percent();
    match cpu_stats_percent {
        Ok(_x) => info!("++ peach-stats cpu_stats_percent endpoint is online"),
        Err(_x) => warn!("++ peach-stats cpu_stats_percent is offline"),
    }

    // cpu_stats_percent
    let disk_usage = stats_client::disk_usage();
    match disk_usage {
        Ok(_x) => info!("++ peach-stats disk_usage endpoint is online"),
        // could include  more detailed info about the specific error here below
        Err(_x) => warn!("++ peach-stats disk_usage endpoint is offline"),
    }

    // questions:
    // how should peach-probe return its results to the user
    // and how does this interact with levels of verbosity?
    // - I assume peach-probe should continue probing after it encounters an error,
    // so it shouldn't just panic and end
    // - should errors be stored in some type of data structure that gets passed around?
    // so that in the end, a sort of final report can be generated of what is online
    // and what is offline?
    // - and/or should there be printed feedback?
    // - for handling levels of verbosity, should we use the logging system (debug/info/warn), and have
    // the level of verbosity simply change the log level
    // or should we write a sort of custom log function, that decides what to do, based
    // on the verbosity that is set
}

use std::env;
use swss_common::types::dbconnector::DEFAULT_SONIC_DB_GLOBAL_CONFIG_FILE;
use swss_common::types::dbconnector::DEFAULT_SONIC_DB_CONFIG_FILE;
use swss_common::sonic_db_config_initialize_global;
use swss_common::sonic_db_config_initialize;
mod sonic_db_cli;
use sonic_db_cli::sonic_db_cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    let global_config_result = sonic_db_config_initialize_global(DEFAULT_SONIC_DB_GLOBAL_CONFIG_FILE, false);
    if global_config_result.is_err() {
        eprintln!("Failed to initialize global config: {}", global_config_result.err().unwrap());
        return;
    }
    let config_result = sonic_db_config_initialize(DEFAULT_SONIC_DB_CONFIG_FILE);
    if config_result.is_err() {
        eprintln!("Failed to initialize config: {}", config_result.err().unwrap());
        return;
    }

    sonic_db_cli(args);
}

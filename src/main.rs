
mod config;
mod database;
mod cmc_collector;
mod scheduler;
mod types;
use std::env;

use cmc_collector::CmcCollector;
use scheduler::Scheduler;
use config::Config;
use env_logger;
use database::Database;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    Config::init("./config/prod.json".to_string());
    let mut kusama_db = Database::new(Config::current().db_address.parse().unwrap(),
        Config::current().db_port, Config::current().kusama_db_name.as_str());
    let result = kusama_db.connect().await;
    let mut polkadot_db = Database::new(Config::current().db_address.parse().unwrap(),
        Config::current().db_port, Config::current().polkadot_db_name.as_str());
    let result = polkadot_db.connect().await;
    let cmc_collector = CmcCollector::new(Config::current().coinmarketcap_key.to_string());
    let cmc_collector_ksm = cmc_collector.clone();
    let mut scheduler = Scheduler::new(cmc_collector_ksm, kusama_db, "KSM".to_string());
    let ksm = scheduler.start();
    let mut scheduler = Scheduler::new(cmc_collector.clone(), polkadot_db, "DOT".to_string());
    let dot = scheduler.start();
    let ksm = ksm.await;
    let dot = dot.await;
    ksm.await;
    dot.await;
}

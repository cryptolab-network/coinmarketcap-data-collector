use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub db_address: String,
    pub db_port: u16,
    pub kusama_db_name: String,
    pub polkadot_db_name: String,
    pub db_has_credential: bool,
    pub db_username: Option<String>,
    pub db_password: Option<String>,
    pub coinmarketcap_key: String,
}

impl Config {
    pub fn init(path: String) {
        let config = read_config(path).unwrap();
        config.make_current();
    }
    pub fn current() -> Arc<Config> {
        CURRENT_CONFIG.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_CONFIG.with(|c| *c.write().unwrap() = Arc::new(self))
    }
}

thread_local! {
    static CURRENT_CONFIG: RwLock<Arc<Config>> = RwLock::new(Default::default());
}


fn read_config(path: String) -> Result<Config, Box<dyn Error>> {
    
    let file = File::open(path);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let config: Config = serde_json::from_reader(reader)?;
            Ok(config)
        }
        Err(e) => Err(Box::new(e))
    }
}

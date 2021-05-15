
use std::collections::HashMap;

use serde::Deserialize;
use serde_json::{Value};

#[derive(Deserialize, Debug)]
pub struct CmcResponse {
  pub data: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Price {
    pub name: String,
    pub symbol: String,
    pub is_active: i32,
    pub is_fiat: i32,
    pub quote: HashMap<String, Quote>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Quote {
  pub last_updated: String,
  pub price: f64,
}

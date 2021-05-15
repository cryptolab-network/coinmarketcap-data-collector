
use std::collections::HashMap;


use crate::types::{self, Price};
#[derive(Clone)]
pub struct CmcCollector {
  url: String,
  api_key: String
}

impl CmcCollector {
  pub fn new(api_key: String) -> Self {
    CmcCollector {
      url: "https://pro-api.coinmarketcap.com/".to_string(),
      api_key: api_key
    }
  }

  pub async fn get_closing_price(&self, coin: &str) -> Result<Price, reqwest::Error>{
    let client = reqwest::Client::new();
    let res = client.get(self.url.to_string() + "v1/cryptocurrency/quotes/latest")
    .query(&[("symbol", coin)])
    .header("X-CMC_PRO_API_KEY", self.api_key.to_string());
    let resp = res.send().await;
    // println!("{:?}", resp);
    match resp {
      Ok(resp) => {
        // println!("{}", resp.text().await.unwrap());
        let data = resp.json::<types::CmcResponse>().await;
        match data {
            Ok(data) => {
              // println!("{:?}", data.data);
              let d: HashMap<String, Price> = serde_json::from_value(data.data).unwrap();
              let price = d[coin].clone();
              Ok(price)
            },
            Err(err) => {Err(err)},
        }
        // Ok(())
      }
      Err(err) => Err(err)
    }
  }
}
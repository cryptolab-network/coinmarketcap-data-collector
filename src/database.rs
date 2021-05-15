use std::net::Ipv4Addr;

use chrono::NaiveTime;
use mongodb::{Client, error::Error, options::{ClientOptions, CreateCollectionOptions, CreateCollectionOptionsBuilder, FindOneAndUpdateOptions, IndexOptionDefaults}};
use mongodb::bson::{doc};
use crate::{config::Config, types};

#[derive(Debug, Clone)]
pub struct Database {
    ip: Ipv4Addr,
    port: u16,
    db_name: String,
    client: Option<Client>,
}

impl Database {
    pub fn new(ip: Ipv4Addr, port: u16, db_name: &str) -> Self {
        Database {
            ip: ip,
            port: port,
            db_name: db_name.to_string(),
            client: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        let need_credential = Config::current().db_has_credential;
        let mut url = "mongodb://".to_string();
        if need_credential {
            if let Some(username) = Config::current().db_username.to_owned() {
                if let Some(password) = Config::current().db_password.to_owned() {
                    url += format!("{}:{}@", username, password).as_str();
                }
            }
        }
        url += format!("{}:{}/{}", self.ip, self.port, self.db_name).as_str();
        let mut client_options = ClientOptions::parse(url.as_str()).await?;
        // Manually set an option.
        client_options.app_name = Some("cryptolab".to_string());
        // Get a handle to the deployment.
        self.client = Some(Client::with_options(client_options)?);
        Ok(())
    }

    pub async fn save_price(&self, coin: String, price: types::Price) -> Result<(), Error> {
      let quote = &price.quote["USD"];
      let timestamp = chrono::DateTime::parse_from_rfc3339(&quote.last_updated)
      .unwrap().date().and_time(NaiveTime::from_hms(0, 0, 0)).unwrap().timestamp();
      let db = self.client.as_ref().unwrap().database(self.db_name.as_str());
      let options = FindOneAndUpdateOptions::builder()
        .upsert(Some(true))
        .build();
      let result = db.collection::<serde_json::Value>("price").find_one_and_update(doc! {
        "timestamp": timestamp
      }, doc! {
        "$set" : {
          "price": quote.price,
          "timestamp": timestamp
        }
      }, options).await;
      match result {
          Ok(_) => {},
          Err(e) => {println!("{:?}", e)},
      }
      Ok(())
    }
  }
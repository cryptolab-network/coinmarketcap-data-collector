use std::thread;

use crate::cmc_collector::CmcCollector;
use tokio::task::JoinHandle;
use tokio_cron_scheduler::{JobScheduler, Job};
use crate::database::Database;
#[derive(Clone)]
pub struct Scheduler {
  cmc_collector: CmcCollector,
  db: Database,
  coin: String,
}

impl Scheduler {
  pub fn new(cmc_collector: CmcCollector, database: Database, coin: String) -> Self{
    Scheduler {
      cmc_collector: cmc_collector,
      db: database,
      coin: coin,
    }
  }

  pub async fn start(&mut self) -> JoinHandle<()> {
    let this = self.clone();
    let mut sched = JobScheduler::new();
    let coin = self.coin.clone();
    self.handle_closing_price(coin.clone().as_str()).await;
    let _ = sched.add(Job::new("0 0 0 * * ?", move |_uuid, _l| {
      println!("Execute Sched");
      let coin_ = coin.clone();
      let mut this_ = this.clone();
      thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(this_.handle_closing_price(coin_.clone().as_str()));
      }).join().expect("Thread panicked");
    }).unwrap());

    sched.start()
  }

  async fn handle_closing_price(&mut self, coin: &str) {
    println!("Get price of {}", coin);
    let price = self.cmc_collector.get_closing_price(coin).await;
      if let Ok(price) = price {
        self.db.save_price(coin.to_string(), price).await;
      }
  }
}
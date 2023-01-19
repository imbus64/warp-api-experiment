use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::Customer;

pub type Db = Arc<Mutex<Vec<Customer>>>;

use serde_json::from_reader;
use std::fs::File;

pub fn init_db() -> Db {
    let file = File::open("db.json");

    match file {
        Ok(json) => {
            let customers = from_reader(json).unwrap();
            Arc::new(Mutex::new(customers))
        }
        Err(_) => Arc::new(Mutex::new(Vec::new())),
    }
}

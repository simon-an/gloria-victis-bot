use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref RUNNING: Arc<Mutex<bool>> = {
        let running: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
        running
    };
    pub static ref COUNTER: Arc<Mutex<u32>> = {
        let counter: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        counter
    };
}
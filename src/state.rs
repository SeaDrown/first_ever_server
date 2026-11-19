use crate::models::Task;
// Arc allows multiple owners to reference the same data
// Mutex means Mutual exclusion, only one owner can modify data at one time
use std::sync::{Arc, Mutex};

// contains the data that the entire app shares
#[derive(Clone)]
pub struct AppState {
    // Arc<Mutex<>> allows us to reference data from multiple differen owners,
    // while the data can only be modified by one owner at a time
    pub tasks: Arc<Mutex<Vec<Task>>>,
    pub next_id: Arc<Mutex<u32>>,
}

impl AppState {
    pub fn new() -> Self {
        // initialise a new app
        let seed = vec![
            Task {
                id: 1,
                title: "Clean the dishes".to_string(),
                done: false,
            },
            Task {
                id: 2,
                title: "Do assignments".to_string(),
                done: false,
            },
            Task {
                id: 3,
                title: "Larp understanding of low level programming".to_string(),
                done: false,
            },
        ];

        AppState {
            tasks: Arc::new(Mutex::new(seed)),
            next_id: Arc::new(Mutex::new(4)),
        }
    }
}

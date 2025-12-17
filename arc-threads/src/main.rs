use std::{
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug)]
struct Database {
    name: String,
    connections: Vec<u32>,
}

impl Database {
    fn new(name: String) -> Database {
        Database {
            name,
            connections: vec![],
        }
    }
    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}

fn main() {
    let name = String::from("DB1");
    let db = Arc::new(Mutex::new(Database::new(name)));
    let mut handles = vec![];

    for i in 0..10 {
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let mut db_lock = db.lock().unwrap();
            db_lock.connect(i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let db_lock = db.lock().unwrap();
    println!("{db_lock:?}");
}

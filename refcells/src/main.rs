use std::cell::RefCell;
use std::rc::Rc;

struct Database {
    max_connections: i32,
}

struct AuthService {
    db: Rc<RefCell<Database>>,
}

struct ContentService {
    db: Rc<RefCell<Database>>,
}

fn main() {
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100,
    }));
    // Instantiate our services
    let authService = AuthService { db: Rc::clone(&db) };
    let contentService = ContentService { db: Rc::clone(&db) };
    // Rc only allows immutable shared ownership of value, so trying to mutate will not compile like the line below:
    // db.max_connections = 200;
    // we need to use refcell
    let mut r1 = db.borrow_mut();
    // This will panic!
    let r2 = db.borrow_mut();
    r1.max_connections = 200;
    // r2.max_connections = 100;
    // println!("db connections: {}", db.max_connections);
}

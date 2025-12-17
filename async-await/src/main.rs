#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let f = my_function();
    println!("Hello, world 2!");
    f.await
}

enum FutureStateMachine {
    State1,
    State2,
    State3,
}

// "async" is syntactic sugar
async fn my_function() {
    println!("I am an async function");
    let s1: String = read_from_database().await;
    println!("first res: {s1}");
    let s2: String = read_from_database().await;
    println!("second res: {s2}");
}

async fn read_from_database() -> String {
    "DB Result".to_owned()
}

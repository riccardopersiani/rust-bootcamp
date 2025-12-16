use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..20 {
        println!("Main thread: {i}");
    }

    handle.join().unwrap();

    let s = "Let's get Rusty".to_owned();

    let handle2 = thread::spawn(move || {
        println!("{s}");
    });
}

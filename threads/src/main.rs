use std::{sync::mpsc, thread, time::Duration};

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

    let (tx, rx) = mpsc::channel();
    let sentences = [
        "!dlroW olleH".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!ytsuR teG s'teL".to_owned(),
        "!tsuB ro tsuR".to_owned(),
    ];
    for s in sentences {
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let reversed: String = s.chars().rev().collect();
            tx_clone.send(reversed).unwrap();
        });
    }

    drop(tx);
    // rx.recv();
    for sentence in rx {
        println!("{sentence}");
    }
}

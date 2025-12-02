#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 3, y: 4 };

    // this syntax only works if the debug trait is implemented
    println!("{:?}", p1);
    // == it not automatically implemented
    println!("{}", p1 == p2);
    println!("{}", p1 == p3);

    // Not all traits can be derived!
}

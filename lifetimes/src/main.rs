// Concrete vs Generic Lifetimes

// Concrete means value exist at particular memory location and ends when the value is not more at that location

fn main() {
    // here lifetime begins
    let mut s1: String = String::from("Let's get Rusty!");
    let r1: &String = &s1;
    // if this line is moved to the end, the code will not compile!
    println!("{}", r1);
    // here lifetime finishes of r1 finishes, the rust compiler is smart enough to recognize that r1 lifetime is over
    // so a reference "mutable" can be created without compilation errors.
    let r2 = &mut s1;
    r2.push_str("!");
}

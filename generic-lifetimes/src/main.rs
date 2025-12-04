fn main() {
    let player1: String = String::from("player 1");
    let result;
    {
        let player2: String = String::from("player 2");
        result = first_turn(player1.as_str(), player2.as_str());
    }
    println!("Player going first is: {}", result);
}

// the borrow checker cannot analyze this properly returning an error.
// but we can describe relationships lifetime introducing a generic lifetime annotation.
// we define lifetime 'a
fn first_turn_old<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() { p1 } else { p2 }
}

fn first_turn<'a>(p1: &'a str, p2: &str) -> &'a str {
    p1
}

// Error
// The return value is referencing "s", "s" is created inside this function,
// so at the end s will go out of scope and the reference will be invalid
// but with static lifetime things can change
fn first_turn_new<'a>(p1: &'a str, p2: &str) -> &'a str {
    let s = "example".to_owned();
    s.as_ref()
}

// return value must live as long as p1
fn first_turn_new_2<'a>(p1: &'a str, p2: &str) -> &'a str {
    let s: &'static str = "example"; // valid for the whole duration fo the program
    s.as_ref()
}

// Simpler version
// return value must live as long as p1
fn first_turn_new_3(p1: &str, p2: &str) -> &'static str {
    let s: &'static str = "example"; // valid for the whole duration fo the program
    s.as_ref()
}

fn main() {
    let player1: String = String::from("player 1");
    let player2: String = String::from("player 2");

    let result = first_turn(player1.as_str(), player2.as_str());

    println!("Player going first is: {}", result);
}

// the borrow checker cannot analize this properly returning an error.
// but we can describe lifetime introducing a generic lifetime annotation.
// we define lifetime 'a
fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() { p1 } else { p2 }
}

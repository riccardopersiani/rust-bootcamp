use std::error::Error;

struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: Option<String>,
}

// fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
//     let numbers = card.split(" ");
// }

fn main() {
    let card = "123 1234 1234";
    let vec: Vec<&str> = card.split(" ").collect();
    println!("{}", vec[1]);
}

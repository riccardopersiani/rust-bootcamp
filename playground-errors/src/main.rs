#![allow(dead_code)]

use std::{collections::HashMap, error::Error, fmt, io};

use error_stack::{IntoReport, Report, Result, ResultExt};

#[derive(Debug)]
struct ParsePaymentInfoError;

impl fmt::Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Parsing payment info error: invalid info")
    }
}

impl Error for ParsePaymentInfoError {}

#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other,
}

impl fmt::Display for CreditCardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl Error for CreditCardError {}

#[derive(Debug)]
struct Card {
    ccv: u32,
    year: u32,
    month: u32,
    number: u32,
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or_else(|| {
        let msg = format!("no credit card was found for {name}.");
        Report::new(CreditCardError::InvalidInput(msg.clone())).attach_printable(msg.clone())
    })?;
    let card = parse_card(card_string)
        .change_context(CreditCardError::Other)
        .attach_printable(format!("Other error with input {name}"))?;
    Ok(card)
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    println!("card inserted: {}", card);
    let numbers = card
        .split(" ")
        .map(|s| {
            s.parse()
                .report()
                .attach_printable_lazy(|| format!("{s:?} could not be parsed as u32"))
        })
        .collect::<Result<Vec<u32>, _>>()
        .change_context(ParsePaymentInfoError)
        .attach_printable(format!("Failed to parse input as numbers. Input: {card}"))?;

    Ok(numbers)
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    println!("{}", len);
    let expected_len = 4;
    if len != expected_len {
        return Err(Report::new(ParsePaymentInfoError).attach_printable(format!(
            "Incorrect number of elements parsed, expected {expected_len} but got {len}"
        )));
    }

    let ccv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        ccv,
        year,
        month,
        number,
    })
}

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27"),
        ("Bob", "1234567 Dec 27 123"),
    ]);

    println!("Credit Card Parser Program");

    println!("Please enter name: ");
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }

    println!("You typed: {}", name);

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("Credit Card Info: {card:?}"),
        Err(err) => {
            match err.current_context() {
                CreditCardError::InvalidInput(msg) => println!("\n{msg}"),
                CreditCardError::Other => {
                    println!("Something went wrong! Double check your inputs")
                }
            }
            log::error!("\n{err:?}")
        }
    }
}

// Complete the code by implementing `From<ParseIntError>` for `AddError`.

use std::num::ParseIntError;

#[derive(Debug)]
enum AddError {
    ParseError(ParseIntError),
    Overflow,
}

impl From<ParseIntError> for AddError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseError(value)
    }
}

fn add(num1: &str, num2: &str) -> Result<u8, AddError> {
    let num1 = num1.parse::<u8>()?;
    let num2 = num2.parse::<u8>()?;
    num1.checked_add(num2).ok_or(AddError::Overflow)
}

fn main() {
    let (input1, input2) = ("23", "50");
    match add(input1, input2) {
        Ok(res) => println!("{input1} + {input2} = {res}"),
        Err(e) => println!("Error: {e:?}"),
    }
}

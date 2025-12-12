// Modify the functions to propagate the error instead of panicking.

fn factorial(n: u32) -> Result<u32, String> {
    if n == 0 {
        return Ok(1);
    } else if n > 12 {
        // Factorial of values > 12 would overflow u32, so return an error
        return Err(String::from("Input too large"));
    }
    let result = n * factorial(n - 1).unwrap();
    Ok(result)
}

fn print_factorial(n: u32) -> Result<(), String> {
    let result = factorial(n);
    match result {
        Ok(result) => println!("Factorial of {} is: {}", n, result),
        Err(e) => println!("Failed factorial"),
    }

    Ok(())
}

fn main() {
    let n = 13;
    if let Err(err) = print_factorial(n) {
        eprintln!("Error calculating factorial of {}: {}", n, err);
    }
}

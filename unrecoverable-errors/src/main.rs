// panic interrupts program abruptly

use std::{
    error,
    fs::{self, File},
    io::{self, Read},
    num::ParseIntError,
};

fn main() {
    let v = vec!["1", "2", "3"];
    // println!("{}", v[3]);
    // panic!("Horrible error!");
    let file = File::open("example.txt"); //.expect("failed to open file"); convenient for prototype but then errrs should be handled
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            panic!("error opening file");
        }
    };
    let i = parse_file("example.txt");
    match i {
        Ok(i) => println!("{i}"),
        Err(e) => {
            match e {
                ParseFileError::File => {
                    //
                }
                ParseFileError::Parse(e) => {
                    //
                }
            }
        }
    }
}

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("empty".to_owned())
    } else {
        Ok(199)
    }
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut contents = String::new(); //heap allocated
    File::open("example.txt")?.read_to_string(&mut contents)?;
    Ok(contents)
}

struct User {
    firstname: String,
    lastname: String,
}

fn get_initials(user: User) -> Option<String> {
    let first_initial = user.firstname.chars().next()?;
    let last_initial = user.lastname.chars().next()?;
    Some(format!("{}.{}.", first_initial, last_initial))
}

fn read_first_line(filename: &str) -> Option<String> {
    fs::read_to_string(filename)
        .ok()
        .and_then(|s| s.lines().next().map(|s| s.to_owned()))
}

enum ParseFileError {
    File,
    Parse(ParseIntError),
}

fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename).map_err(|e| ParseFileError::File)?;
    let i = s.parse().map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}

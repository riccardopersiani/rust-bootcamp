struct Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T, // to store closure we need to use generics
}

impl<T> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    let validator = |username: &str, password: &str| !username.is_empty() && !password.is_empty();
    let weak_password = "password".to_owned();
    // Fn - immutably borrow in env.
    // FnMut - mutably borrow variables in env. Can change env.
    // FnOnce -Take ownership of variables and can be called only once.
    let validator2 =  /* move */ |username: &str, password: &str| {
        !username.is_empty()
            && !password.is_empty()
            && password.len() > 8
            && password.contains(['!', '@'])
            && password != weak_password
    };
    println!("{weak_password}"); // will error if the closure uses move
    let creds = Credentials {
        username: "Mario".to_owned(),
        password: "!password".to_owned(),
        validator: validator2,
    };

    println!("{}", validate_credentials(&creds.username, &creds.password));
    println!("{}", validator(&creds.username, &creds.password));
    println!("{}", creds.is_valid());
}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}

// alternative to create a vlidation function is to store a validation function in a Closure.
// Closure are anonymous functions which you can store in varialbes or pass as arguments to other fucntions

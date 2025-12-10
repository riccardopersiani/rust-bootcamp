# Idiomatic Recoverable Errors in Rust

Recoverable errors should implement the `Error` trait.

```rust
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

The trait defines a method called `source()`.
Its default implementation returns `None`.
The `'static` trait bound on many error types means that they cannot contain non static references.
The `Error` trait also includes a `backtrace()` method, but it is only available on nightly Rust.

Errors must also implement `Debug`, which describes how the error should be reported to developers, and `Display`, which describes how it should be shown to end users.

First of all, the `Error` trait provides a standardized, semantic way to mark types as errors.
It lives in the standard library.

### How to Structure Custom Error Types


Custom errors can be implemented as structs or enums.

```rust
struct ServerError {
    status_code: u8,
    body: String,
    source: Box<dyn Error>
}
```

A struct is appropriate when you want to attach more contextual information to your error.

```rust
enum APIErrorType {
    UserInputError(String),
    InternalError(Box<dyn Error>)
}
```


Enums allow your code to distinguish between different categories of errors—useful when you want to change behavior depending on the error variant.
This is the pattern you will most commonly see in Rust codebases.

### Mixing Enums and Structs

```rust
struct APIError {
    msg: string,
    source: Option<Box<dyn Error>>,
    err_type: APIErrorType // points to an enum
}
```

This approach is flexible: you can attach rich context while still being able to pattern-match on the error type.

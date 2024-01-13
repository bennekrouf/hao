use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("fmt");

        match self {
            MyError::Io(e) => write!(f, "I/O Error: {}", e),
            MyError::Custom(msg) => write!(f, "Custom Error: {}", msg),
        }
    }
}

impl Error for MyError {}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        println!("from(err: io::Error)");
        MyError::Io(err)
    }
}

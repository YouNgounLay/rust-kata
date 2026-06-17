use std::fmt; 

#[derive(debug)]
enum ParseError {
    EmptyInput, 
    InvalidNumber, 
    UnknownCommand,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::result {
        match self {
            ParseError::EmptyInput => write!(f, "Input was empty"),
            ParseError::InvalidNumber => write!(f, "Invalid Number"),
            ParseError::UnknownCommand => write!(f, "Unknown Command"),
        }
    }
}

impl std::error::Error for ParseError {} // Mark this as an error type

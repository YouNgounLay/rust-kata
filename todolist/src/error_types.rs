use std::fmt; 

#[derive(Debug)]
pub enum ParseError {
    EmptyInput, 
    InvalidNumber, 
    UnknownCommand,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Input was empty"),
            ParseError::InvalidNumber => write!(f, "Invalid Number"),
            ParseError::UnknownCommand => write!(f, "Unknown Command"),
        }
    }
}

impl std::error::Error for ParseError {} // Mar this as an error type

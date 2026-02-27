use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvariantNameError {
    Empty,
    InvalidFormat { value: String },
}

impl fmt::Display for InvariantNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvariantNameError::Empty => write!(f, "invariant name cannot be empty"),
            InvariantNameError::InvalidFormat { value } => {
                write!(f, "invalid invariant name format: {value}")
            }
        }
    }
}

impl Error for InvariantNameError {}

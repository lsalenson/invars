use std::fmt;
use crate::scope::Scope;

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scope::Dataset => write!(f, "dataset"),
            Scope::Column { name } => write!(f, "column '{name}'"),
        }
    }
}
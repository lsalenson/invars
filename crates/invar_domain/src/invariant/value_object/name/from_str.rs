use std::str::FromStr;
use crate::invariant::value_object::name::error::InvariantNameError;
use crate::invariant::value_object::name::InvariantName;

impl FromStr for InvariantName {
    type Err = InvariantNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}
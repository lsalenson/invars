use crate::invariant::value_object::name::InvariantName;
use crate::invariant::value_object::name::error::InvariantNameError;
use std::str::FromStr;

impl FromStr for InvariantName {
    type Err = InvariantNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

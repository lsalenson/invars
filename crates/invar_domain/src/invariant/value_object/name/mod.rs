mod error;
mod from_str;

use crate::invariant::value_object::name::error::InvariantNameError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InvariantName(String);

impl InvariantName {
    pub fn new(value: impl Into<String>) -> Result<Self, InvariantNameError> {
        let raw = value.into().trim().to_lowercase();

        if raw.is_empty() {
            return Err(InvariantNameError::Empty);
        }

        if !raw.chars().all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit()) {
            return Err(InvariantNameError::InvalidFormat { value: raw });
        }

        if raw.starts_with('_') || raw.ends_with('_') {
            return Err(InvariantNameError::InvalidFormat { value: raw });
        }

        if raw.contains("__") {
            return Err(InvariantNameError::InvalidFormat { value: raw });
        }

        Ok(Self(raw))
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

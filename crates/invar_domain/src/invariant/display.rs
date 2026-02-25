use crate::invariant::Invariant;

impl std::fmt::Display for Invariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} [{}] on {}",
            self.id.as_str(),
            self.name.as_str(),
            self.scope
        )
    }
}
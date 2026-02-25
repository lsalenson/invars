mod display;
mod error;

use crate::invariant::Invariant;
use crate::invariant::error::InvariantError;
use crate::invariant::value_object::id::InvariantId;
use crate::spec::error::{SpecError, SpecResult};

use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Spec {
    invariants: Vec<Invariant>,
}

impl Spec {
    pub fn new() -> Self {
        Self {
            invariants: Vec::new(),
        }
    }

    pub fn from_invariants(invariants: Vec<Invariant>) -> Self {
        Self { invariants }
    }

    pub fn invariants(&self) -> &[Invariant] {
        &self.invariants
    }

    pub fn into_invariants(self) -> Vec<Invariant> {
        self.invariants
    }

    pub fn push(&mut self, invariant: Invariant) {
        self.invariants.push(invariant);
    }

    pub fn extend<I>(&mut self, invariants: I)
    where
        I: IntoIterator<Item = Invariant>,
    {
        self.invariants.extend(invariants);
    }

    pub fn is_empty(&self) -> bool {
        self.invariants.is_empty()
    }

    pub fn len(&self) -> usize {
        self.invariants.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Invariant> {
        self.invariants.iter()
    }

    pub fn find_by_id(&self, id: &InvariantId) -> Option<&Invariant> {
        self.invariants.iter().find(|inv| inv.id() == id)
    }
}

impl Default for Spec {
    fn default() -> Self {
        Self::new()
    }
}

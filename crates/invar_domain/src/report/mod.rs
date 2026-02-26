mod display;
mod error;
pub use error::{ReportError, ReportResult};

use crate::severity::Severity;
use crate::violation::Violation;
use std::cmp::PartialEq;

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    violations: Vec<Violation>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            violations: Vec::new(),
        }
    }
    pub fn validate(&self) -> ReportResult<()> {
        for (i, v) in self.violations.iter().enumerate() {
            v.validate()
                .map_err(|e| ReportError::invalid_violation(i, e))?;
        }
        Ok(())
    }
    pub fn from_violations(violations: Vec<Violation>) -> Self {
        Self { violations }
    }

    pub fn violations(&self) -> &[Violation] {
        &self.violations
    }

    pub fn into_violations(self) -> Vec<Violation> {
        self.violations
    }

    pub fn push(&mut self, violation: Violation) {
        self.violations.push(violation);
    }

    pub fn extend<I>(&mut self, violations: I)
    where
        I: IntoIterator<Item = Violation>,
    {
        self.violations.extend(violations);
    }

    pub fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn len(&self) -> usize {
        self.violations.len()
    }

    pub fn failed(&self) -> bool {
        self.violations.iter().any(|v| v.severity().is_error())
    }

    pub fn has_errors(&self) -> bool {
        self.violations
            .iter()
            .any(|v| matches!(v.severity(), Severity::Error))
    }

    pub fn has_warnings(&self) -> bool {
        self.violations
            .iter()
            .any(|v| v.severity() == Severity::Warn)
    }

    pub fn count_by_severity(&self, severity: Severity) -> usize {
        self.violations
            .iter()
            .filter(|v| v.severity() == severity)
            .count()
    }

    pub fn error_count(&self) -> usize {
        self.count_by_severity(Severity::Error)
    }

    pub fn warn_count(&self) -> usize {
        self.count_by_severity(Severity::Warn)
    }

    pub fn info_count(&self) -> usize {
        self.count_by_severity(Severity::Info)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Violation> {
        self.violations.iter()
    }

    pub fn errors(&self) -> impl Iterator<Item = &Violation> {
        self.violations
            .iter()
            .filter(|v| matches!(v.severity(), Severity::Error))
    }

    pub fn warnings(&self) -> impl Iterator<Item = &Violation> {
        self.violations
            .iter()
            .filter(|v| v.severity() == Severity::Warn)
    }
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

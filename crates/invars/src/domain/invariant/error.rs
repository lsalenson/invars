use std::{error::Error, fmt};

use crate::{scope::Scope, severity::Severity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvariantError {
    InvalidId {
        id: String,
    },
    MissingParam {
        invariant_id: String,
        key: String,
    },
    InvalidParam {
        invariant_id: String,
        key: String,
        reason: String,
    },
    UnknownInvariant {
        name: String,
    },
    InvalidScope {
        invariant_id: String,
        scope: Scope,
        expected: String,
    },
    UnexpectedSeverity {
        invariant_id: String,
        expected: Severity,
        found: Severity,
    },
}

impl InvariantError {
    pub fn invalid_id(id: impl Into<String>) -> Self {
        Self::InvalidId { id: id.into() }
    }

    pub fn missing_param(invariant_id: impl Into<String>, key: impl Into<String>) -> Self {
        Self::MissingParam {
            invariant_id: invariant_id.into(),
            key: key.into(),
        }
    }

    pub fn invalid_param(
        invariant_id: impl Into<String>,
        key: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidParam {
            invariant_id: invariant_id.into(),
            key: key.into(),
            reason: reason.into(),
        }
    }

    pub fn unknown(name: impl Into<String>) -> Self {
        Self::UnknownInvariant { name: name.into() }
    }

    pub fn invalid_scope(
        invariant_id: impl Into<String>,
        scope: Scope,
        expected: impl Into<String>,
    ) -> Self {
        Self::InvalidScope {
            invariant_id: invariant_id.into(),
            scope,
            expected: expected.into(),
        }
    }

    pub fn unexpected_severity(
        invariant_id: impl Into<String>,
        expected: Severity,
        found: Severity,
    ) -> Self {
        Self::UnexpectedSeverity {
            invariant_id: invariant_id.into(),
            expected,
            found,
        }
    }
}

impl fmt::Display for InvariantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvariantError::InvalidId { id } => {
                write!(f, "invalid invariant id: {id}")
            }
            InvariantError::MissingParam { invariant_id, key } => {
                write!(
                    f,
                    "invariant '{invariant_id}' is missing required param '{key}'"
                )
            }
            InvariantError::InvalidParam {
                invariant_id,
                key,
                reason,
            } => {
                write!(
                    f,
                    "invalid param '{key}' for invariant '{invariant_id}': {reason}"
                )
            }
            InvariantError::UnknownInvariant { name } => {
                write!(f, "unknown invariant: {name}")
            }
            InvariantError::InvalidScope {
                invariant_id,
                scope,
                expected,
            } => {
                write!(
                    f,
                    "invalid scope for invariant '{invariant_id}': got {scope}, expected {expected}"
                )
            }
            InvariantError::UnexpectedSeverity {
                invariant_id,
                expected,
                found,
            } => {
                write!(
                    f,
                    "unexpected severity for invariant '{invariant_id}': expected {expected}, found {found}"
                )
            }
        }
    }
}

impl Error for InvariantError {}

pub type InvariantResult<T> = Result<T, InvariantError>;

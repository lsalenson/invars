//! # Invars
//!
//! Invars is a data validation framework for Rust.
//!
//! It allows you to declare invariants on datasets and validate them
//! against a Polars DataFrame or LazyFrame.

pub use invar_application::*;
pub use invar_domain::*;
pub use invar_polars::*;

pub mod prelude {
    pub use invar_domain::{invariant::Invariant, scope::Scope, severity::Severity, spec::Spec};

    pub use invar_application::RunSpec;

    pub use invar_polars::PolarsEngine;
}

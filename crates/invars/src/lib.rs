//! # Invars
//!
//! Invars is a data validation framework for Rust.
//!
//! It allows you to declare invariants on datasets and validate them
//! against a Polars DataFrame or LazyFrame.

pub use invars_domain::*;
pub use invars_polars::*;

pub mod prelude {
    pub use invars_domain::{invariant::Invariant, scope::Scope, severity::Severity, spec::Spec};

    pub use invars_application::RunSpec;

    pub use invars_polars::PolarsEngine;
}

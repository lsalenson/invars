use polars::prelude::*;

use invars_application::{ApplicationError, ApplicationResult, Engine};
use invars_domain::report::Report;
use invars_domain::spec::Spec;

use crate::checks;

pub struct PolarsEngine;

impl PolarsEngine {
    pub fn new() -> Self {
        Self
    }
    pub fn execute_lazy(&self, lf: &LazyFrame, spec: &Spec) -> ApplicationResult<Report> {
        let df = lf
            .clone()
            .collect()
            .map_err(|e| ApplicationError::engine_failure(e.to_string()))?;

        self.execute(&df, spec)
    }
}

impl Default for PolarsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine for PolarsEngine {
    type Dataset = DataFrame;

    fn execute(&self, dataset: &Self::Dataset, spec: &Spec) -> ApplicationResult<Report> {
        let mut report = Report::new();

        for invariant in spec.invariants() {
            let violations = checks::run(dataset, invariant)
                .map_err(|e| ApplicationError::engine_failure(e.to_string()))?;

            report.extend(violations);
        }

        Ok(report)
    }
}

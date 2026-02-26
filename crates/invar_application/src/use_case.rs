use invar_domain::report::Report;
use invar_domain::spec::Spec;

use crate::engine::Engine;
use crate::error::{ApplicationError, ApplicationResult};

pub struct RunSpec<E: Engine> {
    engine: E,
}

impl<E: Engine> RunSpec<E> {
    pub fn new(engine: E) -> Self {
        Self { engine }
    }

    pub fn run(&self, dataset: &E::Dataset, spec: &Spec) -> ApplicationResult<Report> {
        spec.validate().map_err(ApplicationError::InvalidSpec)?;

        let report = self.engine.execute(dataset, spec)?;

        report.validate().map_err(ApplicationError::InvalidReport)?;

        Ok(report)
    }
}

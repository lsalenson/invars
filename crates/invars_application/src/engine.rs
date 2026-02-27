use invars_domain::report::Report;
use invars_domain::spec::Spec;

use crate::error::ApplicationResult;

pub trait Engine {
    type Dataset;

    fn execute(&self, dataset: &Self::Dataset, spec: &Spec) -> ApplicationResult<Report>;
}

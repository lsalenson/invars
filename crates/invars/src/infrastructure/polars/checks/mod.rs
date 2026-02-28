use polars::prelude::*;

use crate::invariant::Invariant;
use crate::violation::Violation;
use crate::infrastructure::polars::kind::PolarsKind;

mod not_null;
mod row_count_min;
mod unique;

pub type CheckResult = Result<Vec<Violation>, Box<dyn std::error::Error>>;

pub fn run(
    df: &DataFrame,
    invariant: &Invariant<PolarsKind>,
) -> CheckResult {
    match invariant.kind() {
        PolarsKind::NotNull => not_null::check(df, invariant),
        PolarsKind::Unique => unique::check(df, invariant),
        PolarsKind::RowCountMin => row_count_min::check(df, invariant),
    }
}

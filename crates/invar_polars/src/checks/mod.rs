use polars::prelude::*;

use invar_domain::invariant::Invariant;
use invar_domain::violation::Violation;

mod not_null;
mod row_count_min;
mod unique;

pub type CheckResult = Result<Vec<Violation>, Box<dyn std::error::Error>>;

pub fn run(df: &DataFrame, invariant: &Invariant) -> CheckResult {
    match invariant.name().as_str() {
        "not_null" => not_null::check(df, invariant),
        "unique" => unique::check(df, invariant),
        "row_count_min" => row_count_min::check(df, invariant),
        other => Err(format!("unknown invariant: {other}").into()),
    }
}

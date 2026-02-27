use polars::prelude::*;

use invar_domain::invariant::Invariant;
use invar_domain::scope::Scope;
use invar_domain::violation::Violation;
use invar_domain::violation::value_object::metric_value::MetricValue;

use crate::checks::CheckResult;

pub fn check(df: &DataFrame, inv: &Invariant) -> CheckResult {
    let column = match inv.scope() {
        Scope::Column { name } => name,
        _ => return Err("unique requires column scope".into()),
    };

    let series = df.column(column)?;
    let total = series.len() as i64;
    let unique = series.n_unique()? as i64;
    let duplicates = total - unique;

    if duplicates <= 0 {
        return Ok(vec![]);
    }

    let violation = Violation::new(
        inv.id().clone(),
        inv.severity(),
        inv.scope().clone(),
        format!("found {duplicates} duplicate value(s)"),
    )
    .with_metric("duplicate_count", MetricValue::Int(duplicates));

    Ok(vec![violation])
}

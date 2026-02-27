use polars::prelude::*;

use crate::infrastructure::polars::checks::CheckResult;
use crate::invariant::Invariant;
use crate::scope::Scope;
use crate::violation::value_object::metric_value::MetricValue;
use crate::violation::Violation;

pub fn check(df: &DataFrame, inv: &Invariant) -> CheckResult {
    let column = match inv.scope() {
        Scope::Column { name } => name,
        _ => return Err("not_null requires column scope".into()),
    };

    let series = df.column(column)?;
    let null_count = series.null_count() as i64;

    if null_count == 0 {
        return Ok(vec![]);
    }

    let violation = Violation::new(
        inv.id().clone(),
        inv.severity(),
        inv.scope().clone(),
        format!("found {null_count} null value(s)"),
    )
    .with_metric("null_count", MetricValue::Int(null_count));

    Ok(vec![violation])
}

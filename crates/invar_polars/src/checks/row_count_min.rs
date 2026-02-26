use polars::prelude::*;

use invar_domain::invariant::Invariant;
use invar_domain::scope::Scope;
use invar_domain::violation::Violation;
use invar_domain::violation::value_object::metric_value::MetricValue;

use crate::checks::CheckResult;

pub fn check(df: &DataFrame, inv: &Invariant) -> CheckResult {
    if !matches!(inv.scope(), Scope::Dataset) {
        return Err("row_count_min requires dataset scope".into());
    }

    let min_str = inv.param("min").ok_or("missing param 'min'")?;
    let min: i64 = min_str.parse()?;

    let rows = df.height() as i64;

    if rows >= min {
        return Ok(vec![]);
    }

    let violation = Violation::new(
        inv.id().clone(),
        inv.severity(),
        inv.scope().clone(),
        format!("row count {rows} is below minimum {min}"),
    )
        .with_metric("row_count", MetricValue::Int(rows))
        .with_metric("min", MetricValue::Int(min));

    Ok(vec![violation])
}
use polars::prelude::*;
use polars::prelude::AnyValue;
use crate::invariant::Invariant;
use crate::violation::Violation;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::violation::value_object::metric_value::MetricValue;

/// Simple custom expression that must evaluate to boolean per-row.
/// We count rows where expression == true.
pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {

    let column = inv.require_param("column").ok()?;

    Some(
        col(column)
            .cast(DataType::Boolean)
            .eq(lit(false))
            .sum()
    )
}

pub fn map(inv: &Invariant<PolarsKind>, value: AnyValue) -> Option<Violation> {
    let count = value.try_extract::<i64>().ok()?;

    if count > 0 {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("custom expression failed on {count} rows"),
            )
                .with_metric("failure_count", MetricValue::Int(count))
        )
    } else {
        None
    }
}
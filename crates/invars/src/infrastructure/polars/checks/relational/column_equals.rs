use polars::prelude::*;
use polars::prelude::AnyValue;
use crate::invariant::Invariant;
use crate::violation::Violation;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::scope::Scope;
use crate::violation::value_object::metric_value::MetricValue;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else { return None };
    let other = inv.require_param("other_column").ok()?;

    Some(
        col(name)
            .neq(col(other))
            .sum()
    )
}

pub fn map(inv: &Invariant<PolarsKind>, value: AnyValue) -> Option<Violation> {
    let mismatch_count = value.try_extract::<i64>().ok()?;

    if mismatch_count > 0 {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("{mismatch_count} mismatching rows"),
            )
                .with_metric("mismatch_count", MetricValue::Int(mismatch_count))
        )
    } else {
        None
    }
}
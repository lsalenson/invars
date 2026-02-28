use polars::prelude::*;
use polars::prelude::AnyValue;
use crate::invariant::Invariant;
use crate::violation::Violation;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::scope::Scope;
use crate::violation::value_object::metric_value::MetricValue;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else { return None };

    let condition_column = inv.require_param("condition_column").ok()?;
    let condition_value = inv.require_param("condition_value").ok()?;

    Some(
        col(condition_column)
            .eq(lit(condition_value))
            .and(col(name).is_null())
            .sum()
    )
}

pub fn map(inv: &Invariant<PolarsKind>, value: AnyValue) -> Option<Violation> {
    let violation_count = value.try_extract::<i64>().ok()?;

    if violation_count > 0 {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("{violation_count} rows violate conditional not null rule"),
            )
                .with_metric("violation_count", MetricValue::Int(violation_count))
        )
    } else {
        None
    }
}
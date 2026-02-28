use polars::prelude::*;
use polars::prelude::AnyValue;
use crate::invariant::Invariant;
use crate::violation::Violation;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::scope::Scope;
use crate::violation::value_object::metric_value::MetricValue;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else { return None };

    Some(col(name).n_unique())
}

pub fn map(inv: &Invariant<PolarsKind>, value: AnyValue) -> Option<Violation> {
    let unique_count = value.try_extract::<i64>().ok()?;

    let max_ratio: f64 = inv.require_param("max_ratio").ok()?.parse().ok()?;

    let total_rows: i64 = inv.require_param("row_count_cache").ok()?.parse().ok()?;

    if total_rows == 0 {
        return None;
    }

    let duplicate_count = total_rows - unique_count;
    let ratio = duplicate_count as f64 / total_rows as f64;

    if ratio > max_ratio {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("duplicate ratio {:.4} > {:.4}", ratio, max_ratio),
            )
                .with_metric("duplicate_ratio", MetricValue::Float(ratio))
                .with_metric("duplicate_count", MetricValue::Int(duplicate_count)),
        )
    } else {
        None
    }
}
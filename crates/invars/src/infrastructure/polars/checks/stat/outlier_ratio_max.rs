use polars::prelude::*;
use polars::prelude::AnyValue;
use crate::invariant::Invariant;
use crate::violation::Violation;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::scope::Scope;
use crate::violation::value_object::metric_value::MetricValue;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else { return None };

    let z: f64 = inv.require_param("z").ok()?.parse().ok()?;

    Some(
        (
            (col(name) - col(name).mean())
                / col(name).std(1)
        )
            .abs()
            .gt(lit(z))
            .sum()
    )
}

pub fn map(inv: &Invariant<PolarsKind>, value: AnyValue) -> Option<Violation> {
    let outlier_count = value.try_extract::<i64>().ok()?;

    let total: i64 = inv.require_param("row_count_cache").ok()?.parse().ok()?;
    let max_ratio: f64 = inv.require_param("max_ratio").ok()?.parse().ok()?;

    if total == 0 {
        return None;
    }

    let ratio = outlier_count as f64 / total as f64;

    if ratio > max_ratio {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("outlier ratio {:.4} > {:.4}", ratio, max_ratio),
            )
                .with_metric("outlier_ratio", MetricValue::Float(ratio))
                .with_metric("outlier_count", MetricValue::Int(outlier_count))
        )
    } else {
        None
    }
}
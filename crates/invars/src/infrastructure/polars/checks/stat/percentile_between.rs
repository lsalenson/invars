use crate::infrastructure::polars::kind::PolarsKind;
use crate::invariant::Invariant;
use crate::scope::Scope;
use crate::violation::Violation;
use crate::violation::value_object::metric_value::MetricValue;
use polars::prelude::AnyValue;
use polars::prelude::*;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else {
        return None;
    };

    let p: f64 = inv.require_param("p").ok()?.parse().ok()?;

    Some(col(name).quantile(lit(p), QuantileMethod::Nearest))
}

pub fn map(inv: &Invariant<PolarsKind>, value: AnyValue) -> Option<Violation> {
    let percentile = value.try_extract::<f64>().ok()?;

    let min: f64 = inv.require_param("min").ok()?.parse().ok()?;
    let max: f64 = inv.require_param("max").ok()?.parse().ok()?;

    if percentile < min || percentile > max {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("percentile out of range: {}", percentile),
            )
            .with_metric("percentile_value", MetricValue::Float(percentile)),
        )
    } else {
        None
    }
}

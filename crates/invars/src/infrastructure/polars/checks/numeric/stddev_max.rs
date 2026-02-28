use crate::infrastructure::polars::kind::PolarsKind;
use crate::invariant::Invariant;
use crate::scope::Scope;
use crate::violation::Violation;
use crate::violation::value_object::metric_value::MetricValue;
use polars::prelude::*;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else {
        return None;
    };

    Some(col(name).cast(DataType::Float64).std(1))
}
pub fn map(inv: &Invariant<PolarsKind>, value: AnyValue) -> Option<Violation> {
    let std = value.try_extract::<f64>().ok()?;
    let max: f64 = inv.require_param("max").ok()?.parse().ok()?;

    if std > max {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("std deviation {std} exceeds max {max}"),
            )
            .with_metric("std_dev", MetricValue::Float(std)),
        )
    } else {
        None
    }
}

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
    let values = inv.require_param("values").ok()?;

    let forbidden: Vec<&str> = values.split(',').collect();
    let series = Series::new(PlSmallStr::from(""), forbidden);

    Some(col(name).is_in(lit(series).implode(), false).sum())
}

pub fn map(inv: &Invariant<PolarsKind>, value: AnyValue) -> Option<Violation> {
    let violation_count = value.try_extract::<i64>().ok()?;

    if violation_count > 0 {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("{violation_count} forbidden values detected"),
            )
            .with_metric("violation_count", MetricValue::Int(violation_count)),
        )
    } else {
        None
    }
}

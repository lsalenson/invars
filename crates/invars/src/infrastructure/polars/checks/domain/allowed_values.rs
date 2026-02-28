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

    let allowed: Vec<&str> = values.split(',').collect();
    let series = Series::new(PlSmallStr::from(""), allowed);

    Some(col(name).is_in(lit(series).implode(), false).not().sum())
}

pub fn map(inv: &Invariant<PolarsKind>, value: AnyValue) -> Option<Violation> {
    let invalid_count = value.try_extract::<i64>().ok()?;

    if invalid_count > 0 {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("{invalid_count} values outside allowed domain"),
            )
            .with_metric("invalid_count", MetricValue::Int(invalid_count)),
        )
    } else {
        None
    }
}

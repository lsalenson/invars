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
    let pattern = inv.require_param("pattern").ok()?;

    Some(
        col(name)
            .cast(DataType::String)
            .str()
            .extract(lit(pattern), 0)
            .is_null()
            .sum(),
    )
}

pub fn map(inv: &Invariant<PolarsKind>, value: AnyValue) -> Option<Violation> {
    let invalid_count = value.try_extract::<i64>().ok()?;

    if invalid_count > 0 {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("{invalid_count} values do not match regex"),
            )
            .with_metric("invalid_count", MetricValue::Int(invalid_count)),
        )
    } else {
        None
    }
}

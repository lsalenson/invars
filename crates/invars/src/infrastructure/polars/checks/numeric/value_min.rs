use polars::prelude::*;
use polars::prelude::AnyValue;
use crate::invariant::Invariant;
use crate::violation::Violation;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::scope::Scope;
use crate::violation::value_object::metric_value::MetricValue;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else { return None };
    let min: f64 = inv.require_param("min").ok()?.parse().ok()?;

    Some(
        col(name)
            .cast(DataType::Float64)
            .lt(lit(min))
            .sum()
    )
}


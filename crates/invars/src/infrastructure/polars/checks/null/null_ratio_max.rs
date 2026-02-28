use crate::infrastructure::polars::kind::PolarsKind;
use crate::invariant::Invariant;
use crate::scope::Scope;
use crate::violation::Violation;
use polars::datatypes::AnyValue;
use polars::prelude::{Expr, col};

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else {
        return None;
    };
    Some(col(name).is_null().sum())
}

pub fn map(inv: &Invariant<PolarsKind>, v: AnyValue) -> Option<Violation> {
    let nulls = v.try_extract::<i64>().ok()?;
    let total: i64 = inv.require_param("row_count_cache").ok()?.parse().ok()?; // or inject

    let ratio = nulls as f64 / total as f64;
    let max: f64 = inv.require_param("max_ratio").ok()?.parse().ok()?;

    if ratio > max {
        Some(Violation::new(
            inv.id().clone(),
            inv.severity(),
            inv.scope().clone(),
            format!("null ratio {ratio} > {max}"),
        ))
    } else {
        None
    }
}

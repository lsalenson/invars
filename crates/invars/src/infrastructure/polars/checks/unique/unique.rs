use polars::datatypes::AnyValue;
use polars::prelude::{col, Expr};
use crate::infrastructure::polars::kind::PolarsKind;
use crate::infrastructure::polars::utils::metric_violation;
use crate::invariant::Invariant;
use crate::scope::Scope;
use crate::violation::Violation;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else { return None };
    Some(col(name).n_unique())
}

pub fn map(inv: &Invariant<PolarsKind>, v: AnyValue) -> Option<Violation> {
    let unique = v.try_extract::<i64>().ok()?;
    let total: i64 = inv.require_param("row_count_cache").ok()?.parse().ok()?;
    metric_violation::<PolarsKind>(inv, "duplicate_count", total - unique, format!("duplicates detected"))
}
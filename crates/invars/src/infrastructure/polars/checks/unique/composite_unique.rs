use crate::infrastructure::polars::kind::PolarsKind;
use crate::infrastructure::polars::utils::metric_violation;
use crate::invariant::Invariant;
use crate::violation::Violation;
use polars::datatypes::AnyValue;
use polars::prelude::{Expr, as_struct, col};

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let cols = inv.require_param("columns").ok()?;
    let cols: Vec<_> = cols.split(',').collect();

    Some(as_struct(cols.iter().map(|c| col(*c)).collect::<Vec<_>>()).n_unique())
}

pub fn map(inv: &Invariant<PolarsKind>, v: AnyValue) -> Option<Violation> {
    let unique = v.try_extract::<i64>().ok()?;
    let total: i64 = inv.require_param("row_count_cache").ok()?.parse().ok()?;
    metric_violation::<PolarsKind>(
        inv,
        "duplicate_count",
        total - unique,
        format!("duplicates detected"),
    )
}

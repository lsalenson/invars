use crate::infrastructure::polars::kind::PolarsKind;
use crate::invariant::Invariant;
use crate::scope::Scope;
use polars::prelude::*;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else {
        return None;
    };
    let max: i64 = inv.require_param("max").ok()?.parse().ok()?;

    Some(col(name).str().len_chars().gt(lit(max)).sum())
}

use polars::prelude::*;
use crate::invariant::Invariant;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::scope::Scope;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else { return None };
    let min: i64 = inv.require_param("min").ok()?.parse().ok()?;

    Some(
        col(name)
            .str()
            .len_chars()
            .lt(lit(min))
            .sum()
    )
}


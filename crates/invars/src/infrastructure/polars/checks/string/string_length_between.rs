use crate::infrastructure::polars::kind::PolarsKind;
use crate::invariant::Invariant;
use crate::scope::Scope;
use polars::prelude::*;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else {
        return None;
    };

    let min: i64 = inv.require_param("min").ok()?.parse().ok()?;
    let max: i64 = inv.require_param("max").ok()?.parse().ok()?;

    let len = col(name).str().len_chars();

    let too_short = len.clone().lt(lit(min));
    let too_long = len.gt(lit(max));

    Some(too_short.or(too_long).sum())
}

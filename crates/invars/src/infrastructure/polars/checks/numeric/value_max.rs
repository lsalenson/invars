use crate::infrastructure::polars::kind::PolarsKind;
use crate::invariant::Invariant;
use crate::scope::Scope;
use polars::prelude::*;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else {
        return None;
    };
    let max: f64 = inv.require_param("max").ok()?.parse().ok()?;

    Some(col(name).cast(DataType::Float64).gt(lit(max)).sum())
}

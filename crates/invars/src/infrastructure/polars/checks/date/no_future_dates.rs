use crate::infrastructure::polars::kind::PolarsKind;
use crate::invariant::Invariant;
use crate::scope::Scope;
use chrono::Utc;
use polars::prelude::*;
pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else {
        return None;
    };

    let today = Utc::now().date_naive();

    Some(col(name).gt(lit(today)).sum())
}

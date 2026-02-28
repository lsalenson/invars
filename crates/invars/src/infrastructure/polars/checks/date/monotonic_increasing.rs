use polars::prelude::*;
use polars::series::ops::NullBehavior;
use crate::invariant::Invariant;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::scope::Scope;
pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else { return None };

    Some(
        col(name)
            .diff(Expr::from(1), NullBehavior::Ignore)
            .lt(lit(0))
            .sum()
    )
}
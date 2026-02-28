use crate::infrastructure::polars::kind::PolarsKind;
use crate::invariant::Invariant;
use crate::scope::Scope;
use polars::prelude::*;
use polars::series::ops::NullBehavior;
pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else {
        return None;
    };

    Some(
        col(name)
            .diff(Expr::from(1), NullBehavior::Ignore)
            .neq(lit(1))
            .sum(),
    )
}

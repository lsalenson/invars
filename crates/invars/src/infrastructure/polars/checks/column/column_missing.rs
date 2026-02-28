use polars::frame::DataFrame;
use crate::invariant::Invariant;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::scope::Scope;
use crate::violation::Violation;

pub fn run_direct(
    df: &DataFrame,
    inv: &Invariant<PolarsKind>,
) -> Option<Violation> {
    if !matches!(inv.kind(), PolarsKind::ColumnMissing) {
        return None;
    }

    let Scope::Column { name } = inv.scope() else { return None };

    if df.column(name).is_ok() {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("column '{name}' should be missing"),
            )
        )
    } else {
        None
    }
}
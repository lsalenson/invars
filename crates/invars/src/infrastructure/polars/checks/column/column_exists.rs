use polars::frame::DataFrame;
use crate::invariant::Invariant;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::violation::Violation;
use crate::scope::Scope;


pub fn run_direct(
    df: &DataFrame,
    inv: &Invariant<PolarsKind>,
) -> Option<Violation> {
    if !matches!(inv.kind(), PolarsKind::ColumnExists) {
        return None;
    }

    let Scope::Column { name } = inv.scope() else { return None };

    if df.column(name).is_err() {
        Some(
            Violation::new(
                inv.id().clone(),
                inv.severity(),
                inv.scope().clone(),
                format!("column '{name}' does not exist"),
            )
        )
    } else {
        None
    }
}
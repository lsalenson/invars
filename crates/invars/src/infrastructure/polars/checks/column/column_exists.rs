use crate::infrastructure::polars::kind::PolarsKind;
use crate::invariant::Invariant;
use crate::scope::Scope;
use crate::violation::Violation;
use polars::frame::DataFrame;

pub fn run_direct(df: &DataFrame, inv: &Invariant<PolarsKind>) -> Option<Violation> {
    if !matches!(inv.kind(), PolarsKind::ColumnExists) {
        return None;
    }

    let Scope::Column { name } = inv.scope() else {
        return None;
    };

    if df.column(name).is_err() {
        Some(Violation::new(
            inv.id().clone(),
            inv.severity(),
            inv.scope().clone(),
            format!("column '{name}' does not exist"),
        ))
    } else {
        None
    }
}

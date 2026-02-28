use crate::infrastructure::polars::kind::PolarsKind;
use crate::invariant::Invariant;
use crate::scope::Scope;
use crate::violation::Violation;
use polars::frame::DataFrame;

pub fn run_direct(df: &DataFrame, inv: &Invariant<PolarsKind>) -> Option<Violation> {
    if !matches!(inv.kind(), PolarsKind::DTypeIs) {
        return None;
    }

    let Scope::Column { name } = inv.scope() else {
        return None;
    };

    let expected = inv.require_param("dtype").ok()?;
    let actual = df.column(name).ok()?.dtype().to_string();

    if actual != expected {
        Some(Violation::new(
            inv.id().clone(),
            inv.severity(),
            inv.scope().clone(),
            format!("dtype '{actual}' != expected '{expected}'"),
        ))
    } else {
        None
    }
}

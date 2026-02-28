use crate::infrastructure::polars::kind::PolarsKind;
use crate::invariant::Invariant;
use crate::violation::Violation;
use polars::frame::DataFrame;
pub fn run_direct(df: &DataFrame, inv: &Invariant<PolarsKind>) -> Option<Violation> {
    if !matches!(inv.kind(), PolarsKind::SchemaEquals) {
        return None;
    }

    let expected = inv.require_param("schema").ok()?;

    let actual = df
        .columns()
        .iter()
        .map(|c| format!("{}:{}", c.name(), c.dtype()))
        .collect::<Vec<_>>()
        .join(",");

    if actual != expected {
        Some(Violation::new(
            inv.id().clone(),
            inv.severity(),
            inv.scope().clone(),
            "schema mismatch".to_string(),
        ))
    } else {
        None
    }
}

use polars::prelude::*;


use crate::invariant::Invariant;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::infrastructure::polars::utils::metric_violation;
use crate::violation::Violation;
pub fn plan_row_count() -> Option<Expr> {
    Some(lit(1).count())
}
pub fn map_row_count(inv: &Invariant<PolarsKind>, v: AnyValue) -> Option<Violation> {
    let count = v.try_extract::<i64>().ok()?;

    match inv.kind() {
        PolarsKind::RowCountMin => {
            let min: i64 = inv.require_param("min").ok()?.parse().ok()?;
            metric_violation::<PolarsKind>(inv, "row_count", if count < min { count } else { 0 },
                             format!("row_count {count} < {min}"))
        }
        PolarsKind::RowCountMax => {
            let max: i64 = inv.require_param("max").ok()?.parse().ok()?;
            metric_violation::<PolarsKind>(inv, "row_count", if count > max { count } else { 0 },
                             format!("row_count {count} > {max}"))
        }
        PolarsKind::RowCountBetween => {
            let min: i64 = inv.require_param("min").ok()?.parse().ok()?;
            let max: i64 = inv.require_param("max").ok()?.parse().ok()?;
            if count < min || count > max {
                Some(
                    Violation::new(
                        inv.id().clone(),
                        inv.severity(),
                        inv.scope().clone(),
                        format!("row_count {count} not in [{min}, {max}]")
                    )
                    .with_metric("row_count", crate::violation::value_object::metric_value::MetricValue::Int(count)),
                )
            } else {
                None
            }
        }
        _ => None,
    }
}
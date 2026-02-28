use polars::prelude::*;
use polars::prelude::AnyValue;
use chrono::NaiveDate;
use crate::invariant::Invariant;
use crate::violation::Violation;
use crate::infrastructure::polars::kind::PolarsKind;
use crate::scope::Scope;
use crate::violation::value_object::metric_value::MetricValue;

pub fn plan(inv: &Invariant<PolarsKind>) -> Option<Expr> {
    let Scope::Column { name } = inv.scope() else { return None };

    let start = inv.require_param("start").ok()?;
    let end = inv.require_param("end").ok()?;

    let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d").ok()?;
    let end_date = NaiveDate::parse_from_str(end, "%Y-%m-%d").ok()?;

    Some(
        col(name)
            .lt(lit(start_date))
            .or(col(name).gt(lit(end_date)))
            .sum()
    )
}


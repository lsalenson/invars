#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolarsKind {
    NotNull,
    Unique,
    RowCountMin,
}

impl PolarsKind {
    pub const ALL: &'static [PolarsKind] = &[
        PolarsKind::NotNull,
        PolarsKind::Unique,
        PolarsKind::RowCountMin,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotNull => "not_null",
            Self::Unique => "unique",
            Self::RowCountMin => "row_count_min",
        }
    }
}

impl std::str::FromStr for PolarsKind {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "not_null" => Ok(Self::NotNull),
            "unique" => Ok(Self::Unique),
            "row_count_min" => Ok(Self::RowCountMin),
            other => Err(format!("unsupported polars invariant: {other}")),
        }
    }
}
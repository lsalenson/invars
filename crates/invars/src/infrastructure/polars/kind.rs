#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolarsKind {
    // Nullability
    NotNull,
    NullRatioMax,

    // Uniqueness
    Unique,
    CompositeUnique,
    DuplicateRatioMax,

    // Row count
    RowCountMin,
    RowCountMax,
    RowCountBetween,

    // Column structure
    ColumnExists,
    ColumnMissing,
    DTypeIs,
    SchemaEquals,

    // Numeric value constraints
    ValueMin,
    ValueMax,
    ValueBetween,
    MeanBetween,
    StdDevMax,
    SumBetween,

    // Date / time constraints
    DateBetween,
    NoFutureDates,
    MonotonicIncreasing,
    NoGapsInSequence,

    // String constraints
    RegexMatch,
    StringLengthMin,
    StringLengthMax,
    StringLengthBetween,

    // Domain constraints
    AllowedValues,
    ForbiddenValues,

    // Statistical / distribution constraints
    OutlierRatioMax,
    PercentileBetween,

    // Relational constraints
    ForeignKey,
    ColumnEquals,
    ConditionalNotNull,

    // Custom Polars expression
    CustomExpr,
}
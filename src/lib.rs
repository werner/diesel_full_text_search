#[macro_use]
extern crate diesel;

mod types {
    #[allow(deprecated)]
    use diesel::types::{HasSqlType, NotNull};
    use diesel::pg::{Pg, PgTypeMetadata, PgMetadataLookup};

    #[derive(Clone, Copy)] pub struct TsQuery;
    #[derive(Clone, Copy)] pub struct TsVector;

    impl HasSqlType<TsQuery> for Pg {
        fn metadata(_: &PgMetadataLookup) -> PgTypeMetadata {
            PgTypeMetadata {
                oid: 3615,
                array_oid: 3645,
            }
        }
    }

    impl HasSqlType<TsVector> for Pg {
        fn metadata(_: &PgMetadataLookup) -> PgTypeMetadata {
            PgTypeMetadata {
                oid: 3614,
                array_oid: 3643,
            }
        }
    }

    impl NotNull for TsVector {}
    impl NotNull for TsQuery {}
}

#[allow(deprecated)]
mod functions {
    use types::*;
    use diesel::types::*;

    sql_function!(length, length_t, (x: TsVector) -> Integer);
    sql_function!(numnode, numnode_t, (x: TsQuery) -> Integer);
    sql_function!(plainto_tsquery, plain_to_tsquery_t, (x: Text) -> TsQuery);
    sql_function!(querytree, querytree_t, (x: TsQuery) -> Text);
    sql_function!(strip, strip_t, (x: TsVector) -> TsVector);
    sql_function!(to_tsquery, to_tsquery_t, (x: Text) -> TsQuery);
    sql_function!(to_tsvector, to_tsvector_t, (x: Text) -> TsVector);
    sql_function!(ts_headline, ts_headline_t, (x: Text, y: TsQuery) -> Text);
    sql_function!(ts_rank, ts_rank_t, (x: TsVector, y: TsQuery) -> Float);
    sql_function!(ts_rank_cd, ts_rank_cd_t, (x: TsVector, y: TsQuery) -> Float);
}

mod dsl {
    use types::*;
    use diesel::expression::{Expression, AsExpression};
    use diesel::expression::grouped::Grouped;

    mod predicates {
        use types::*;
        use diesel::pg::Pg;

        diesel_infix_operator!(Matches, " @@ ", backend: Pg);
        diesel_infix_operator!(Concat, " || ", TsVector, backend: Pg);
        diesel_infix_operator!(And, " && ", TsQuery, backend: Pg);
        diesel_infix_operator!(Or, " || ", TsQuery, backend: Pg);
        diesel_infix_operator!(Contains, " @> ", backend: Pg);
        diesel_infix_operator!(ContainedBy, " <@ ", backend: Pg);
        diesel_infix_operator!(Distance, " <=> ", backend: Pg);
        diesel_infix_operator!(LeftDistance, " <=| ", backend: Pg);
        diesel_infix_operator!(RightDistance, " |=> ", backend: Pg);
    }

    use self::predicates::*;

    pub type Concat<T, U> = Grouped<predicates::Concat<T, U>>;

    pub trait TsVectorExtensions: Expression<SqlType=TsVector> + Sized {
        fn matches<T: AsExpression<TsQuery>>(self, other: T) -> Matches<Self, T::Expression> {
            Matches::new(self, other.as_expression())
        }

        fn concat<T: AsExpression<TsVector>>(self, other: T) -> Concat<Self, T::Expression> {
            Grouped(predicates::Concat::new(self, other.as_expression()))
        }
    }

    pub trait TsQueryExtensions: Expression<SqlType=TsQuery> + Sized {
        fn matches<T: AsExpression<TsVector>>(self, other: T) -> Matches<Self, T::Expression> {
            Matches::new(self, other.as_expression())
        }

        fn and<T: AsExpression<TsQuery>>(self, other: T) -> And<Self, T::Expression> {
            And::new(self, other.as_expression())
        }

        fn or<T: AsExpression<TsQuery>>(self, other: T) -> Or<Self, T::Expression> {
            Or::new(self, other.as_expression())
        }

        fn contains<T: AsExpression<TsQuery>>(self, other: T) -> Contains<Self, T::Expression> {
            Contains::new(self, other.as_expression())
        }

        fn contained_by<T: AsExpression<TsQuery>>(self, other: T) -> ContainedBy<Self, T::Expression> {
            ContainedBy::new(self, other.as_expression())
        }
    }

    pub trait TsRumExtensions: Expression<SqlType=TsVector> + Sized {
        fn distance<T: AsExpression<TsQuery>>(self, other: T) -> Distance<Self, T::Expression> {
            Distance::new(self, other.as_expression())
        }

        fn left_distance<T: AsExpression<TsVector>>(self, other: T) -> LeftDistance<Self, T::Expression> {
            LeftDistance::new(self, other.as_expression())
        }

        fn right_distance<T: AsExpression<TsVector>>(self, other: T) -> RightDistance<Self, T::Expression> {
            RightDistance::new(self, other.as_expression())
        }
    }

    impl<T: Expression<SqlType=TsVector>> TsVectorExtensions for T {
    }

    impl<T: Expression<SqlType=TsQuery>> TsQueryExtensions for T {
    }

    impl<T: Expression<SqlType=TsVector>> TsRumExtensions for T {
    }
}

pub use self::types::*;
pub use self::functions::*;
pub use self::dsl::*;
// use diesel::deserialize::FromSql;
// use diesel::prelude::*;
// use diesel::{
//     backend::Backend,
//     serialize::{self, Output, ToSql},
//     sql_types::BigInt,
// };

// #[derive(Debug)]
// pub struct ChronoDuration(chrono::Duration);

// impl ToSql<BigInt, diesel::sqlite::Sqlite> for ChronoDuration
// where
//     i64: ToSql<BigInt, diesel::sqlite::Sqlite>,
// {
//     fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> serialize::Result {
//         out.set_value(self.0.num_milliseconds());
//         Ok(serialize::IsNull::No)
//     }
// }

// impl FromSql<BigInt, diesel::sqlite::Sqlite> for ChronoDuration
// where
//     i64: ToSql<BigInt, diesel::sqlite::Sqlite>,
// {
//     fn from_sql(
//         bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
//     ) -> diesel::deserialize::Result<Self> {
//         let milliseconds = i64::from_sql(bytes)?;
//         Ok(ChronoDuration(chrono::Duration::milliseconds(milliseconds)))
//     }
// }

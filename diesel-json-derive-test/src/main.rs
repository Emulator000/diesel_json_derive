mod schema;

use diesel::prelude::*;
use diesel::sql_types::Jsonb;
use diesel::{FromSqlRow, AsExpression};
use diesel_json_derive::DieselJsonb;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Selectable,
)]
#[diesel(table_name = crate::schema::foo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
struct Foo {
    id: String,
    bar: Bar,
}

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow, DieselJsonb)]
#[diesel(sql_type = Jsonb)]
struct Bar {
    x: i32,
}

fn main() {
    unimplemented!()
}

#[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
#[postgres(type_name = "citext")]
pub struct Citext;

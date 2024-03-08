use crate::schema::*;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Selectable, QueryableByName, Debug)]
#[table_name = "lookup"]
pub struct LookupEntry {
    pub title: String,
    pub byteoffset: i32,
    pub length: i32,
}

#[derive(Insertable, Queryable, QueryableByName, Selectable, Serialize, Debug)]
#[table_name = "redirect"]
pub struct RedirectEntry {
    pub redirect_from: String,
    pub redirect_to: String,
}

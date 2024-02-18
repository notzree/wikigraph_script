use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "lookup"]
pub struct LookupEntry {
    pub title: String,
    pub offset: i32,
    pub length: i32,
}

#[derive(Debug, Queryable, Serialize)] // Add this line to import the Serialize trait
pub struct Lookup {
    pub id: i32,
    pub title: String,
    pub offset: i32,
    pub length: i32,
}

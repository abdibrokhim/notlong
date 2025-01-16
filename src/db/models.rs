// src/db/models.rs
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::db::schema::short_urls;

// The main struct that mirrors the DB table.
// (The `Selectable` derive requires Diesel >= 2.0, it helps with typed queries)
#[derive(Queryable, Selectable, Debug)]
#[derive(serde::Serialize)]
#[diesel(table_name = short_urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShortUrl {
    pub id: i32,
    pub original_url: String,
    pub short_code: String,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub created_at: NaiveDateTime,
}

// For inserting new rows
#[derive(Insertable)]
#[diesel(table_name = short_urls)]
pub struct NewShortUrl<'a> {
    pub original_url: &'a str,
    pub short_code: &'a str,
}

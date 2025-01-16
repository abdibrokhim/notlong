// src/db/operations.rs
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use crate::db::models::{ShortUrl, NewShortUrl};
use crate::db::schema::short_urls::dsl::*;

pub fn find_by_short_code(conn: &mut PgConnection, code: &str) -> Result<ShortUrl, DieselError> {
    short_urls
        .filter(short_code.eq(code))
        .select(ShortUrl::as_select())
        .first(conn)
        
}
pub fn find_by_original_url(conn: &mut PgConnection, url: &str) -> Result<ShortUrl, DieselError> {
    short_urls
        .filter(original_url.eq(url))
        .select(ShortUrl::as_select())
        .first(conn)
}

pub fn insert_short_url(
    conn: &mut PgConnection,
    new_short: NewShortUrl
) -> Result<ShortUrl, DieselError> {
    diesel::insert_into(short_urls)
        .values(&new_short)
        .get_result::<ShortUrl>(conn)
}

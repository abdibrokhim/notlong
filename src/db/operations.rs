// src/db/operations.rs
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::dsl::sql;

use crate::db::models::{ShortUrl, NewShortUrl};
use crate::db::schema::short_urls::dsl::*;

use crate::db::schema::connected_wallets::dsl::*;
use crate::db::models::{ConnectedWallet, NewConnectedWallet};

// short urls
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

// web3 stuff
pub fn create_wallet(
    conn: &mut PgConnection,
    wallet_addr: &str,
) -> Result<ConnectedWallet, DieselError> {
    let new_wallet = NewConnectedWallet {
        wallet_address: wallet_addr,
    };

    diesel::insert_into(connected_wallets)
        .values(&new_wallet)
        .get_result(conn)
}

pub fn find_by_wallet_address(
    conn: &mut PgConnection,
    addr: &str,
) -> Result<ConnectedWallet, DieselError> {
    connected_wallets
        .filter(wallet_address.eq(addr))
        .select(ConnectedWallet::as_select())
        .first(conn)
}

// without a check, let it go negative.
pub fn decrement_tries_left(conn: &mut PgConnection, addr: &str) -> Result<ConnectedWallet, DieselError> {
    diesel::update(connected_wallets.filter(wallet_address.eq(addr)))
        .set(tries_left.eq(sql("tries_left - 1")))
        .get_result(conn)
}

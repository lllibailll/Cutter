use crate::DbConn;
use diesel;
use diesel::prelude::*;
use crate::schema::{urls};

#[derive(Queryable)]
pub struct Url {
    pub id: i32,
    pub shorten_url: String,
    pub target: String,
}

impl Url {
    pub fn get_url(shorten_url: String, conn: &DbConn) -> Option<Url> {
        let res = urls::table
            .filter(urls::shorten_url.eq(shorten_url))
            .first::<Url>(&conn.0);

        match res {
            Ok(url) => Some(url),
            Err(_) => {
                None
            }
        }
    }
}

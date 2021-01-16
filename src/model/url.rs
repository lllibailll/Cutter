/*
 * Cutter - Url shortener.
 * Copyright (C) 2021 Ibai (lllibailll)
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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

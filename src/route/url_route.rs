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

use rocket::response::Redirect;
use crate::DbConn;
use crate::model::url::Url;

#[get("/")]
pub fn handle_base_url() -> Redirect {
    Redirect::to(format!("https://github.com/lllibailll/Cutter"))
}

#[get("/<details>")]
pub fn handle_url(conn: DbConn, details: String) -> Redirect {
    let url = Url::get_url(details, &conn);

    if url.is_some() {
        Redirect::to(format!("{}", url.unwrap().target))
    } else {
        Redirect::to(format!("https://ibai.dev/"))
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/", routes![handle_base_url, handle_url])
}

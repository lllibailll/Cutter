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

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod model;
mod route;
mod schema;

#[database("Cutter")]
pub struct DbConn(diesel::MysqlConnection);

fn rocket() -> rocket::Rocket {
    let mut ignited_rocket = rocket::ignite()
        .attach(DbConn::fairing());

    ignited_rocket = route::url_route::mount(ignited_rocket);

    ignited_rocket
}

fn main() {
    rocket().launch();
}

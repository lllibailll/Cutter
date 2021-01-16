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

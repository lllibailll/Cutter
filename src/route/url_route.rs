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

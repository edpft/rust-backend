#[macro_use]
extern crate rocket;

mod routes;
#[cfg(test)]
mod tests;

use rocket::fs::{FileServer, relative};

use rocket_dyn_templates::Template;

use routes::{health_check, index, submit};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/health_check", routes![health_check])
        .mount("/add_client", routes![index, submit])
        .attach(Template::fairing())
        .mount("/add_client", FileServer::from(relative!("/static")))
}

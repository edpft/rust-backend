#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;
mod routes;

use routes::health_check;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![health_check])
}

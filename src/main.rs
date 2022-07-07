#[macro_use]
extern crate rocket;

mod routes;
#[cfg(test)]
mod tests;

use routes::health_check;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check])
}

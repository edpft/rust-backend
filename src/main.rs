#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;
#[cfg(test)]
mod tests;

use dotenv::dotenv;
use std::env;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use sqlx::postgres::PgPoolOptions;

use routes::{add_client, get_client, health_check, index};

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let _ = rocket::build()
        .mount("/health_check", routes![health_check])
        .mount("/client", routes![get_client])
        .mount("/add_client", routes![index, add_client])
        .mount("/add_client", FileServer::from(relative!("/static")))
        .manage(pool)
        .attach(Template::fairing())
        .launch()
        .await;

    Ok(())
}

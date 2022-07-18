use rocket::form::{Context, Contextual, Form};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::response::status::NotFound;
use rocket::{post, State};
use rocket_dyn_templates::Template;
use sqlx::postgres::PgPool;

use crate::models::{NewClient, Client};

#[get("/")]
pub async fn health_check() -> Status {
    Status::Ok
}

#[get("/")]
pub async fn index() -> Template {
    Template::render("add_client", &Context::default())
}

#[post("/", data = "<client>")]
pub async fn add_client<'r>(
    pool: &'r State<PgPool>,
    client: Form<Contextual<'r, NewClient<'r>>>,
) -> Result<Redirect, Template> {
    match &client.value {
        Some(client) => {
            let new_client_id = client.add(pool.inner()).await.expect("Add client to database");
            Ok(Redirect::to(format!("/client/{}", new_client_id)))
        }
        None => {
            let template = Template::render("add_client", &client.context);
            Err(template)
        }
    }
}

#[get("/<id>")]
pub async fn get_client<'r>(pool: &'r State<PgPool>, id: i64) -> Result<Template, NotFound<String>> {
    let client = Client::get(pool.inner(), id).await;
    
    match client {
        Ok(client) => Ok(Template::render("client",  client)),
        Err(error) => Err(NotFound(error.to_string()))
    }
}

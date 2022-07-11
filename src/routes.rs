use rocket::form::{Context, Contextual, Form, FromForm};
use rocket::http::Status;
use rocket::post;
use rocket_dyn_templates::Template;

#[get("/")]
pub async fn health_check() -> Status {
    Status::Ok
}

#[derive(Debug, FromForm)]
pub struct Client<'v> {
    #[field(validate = len(1..).or_else(msg!("Enter the client's first name")))]
    pub first_name: &'v str,
    #[field(validate = len(1..).or_else(msg!("Enter the client's last name")))]
    pub last_name: &'v str,
    #[field(validate = contains('@').or_else(msg!("Enter a valid email address")))]
    pub email_address: &'v str,
    pub telephone_number: &'v str,
}

#[get("/")]
pub async fn index() -> Template {
    Template::render("index", &Context::default())
}

#[post("/", data = "<client>")]
pub async fn submit<'r>(client: Form<Contextual<'r, Client<'r>>>) -> (Status, Template) {
    let template = Template::render("index", &client.context);

    (client.context.status(), template)
}

use chrono::{DateTime, Utc};
use rocket::form::FromForm;
use rocket::serde::{Deserialize, Serialize};
use sqlx::{self, postgres::PgPool};

#[derive(Debug, FromForm, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewClient<'v> {
    #[field(validate = len(1..).or_else(msg!("Enter the client's first name")))]
    pub first_name: &'v str,
    #[field(validate = len(1..).or_else(msg!("Enter the client's last name")))]
    pub last_name: &'v str,
    #[field(validate = contains('@').or_else(msg!("Enter a valid email address")))]
    pub email_address: &'v str,
    pub telephone_number: &'v str,
}

pub struct NewClientId {
    id: i64,
}

impl NewClient<'_> {
    pub async fn add(&self, pool: &PgPool) -> Result<i64, sqlx::Error> {
        let new_client_id = sqlx::query_file_as!(
            NewClientId,
            "queries/add_client.sql",
            self.first_name,
            self.last_name,
            self.email_address,
            self.telephone_number
        )
        .fetch_one(pool)
        .await?;

        Ok(new_client_id.id)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Client {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub telephone_number: String,
    pub added_at: DateTime<Utc>,
}

impl Client {
    pub async fn get(pool: &PgPool, id: i64) -> Result<Client, sqlx::Error> {
        let client = sqlx::query_file_as!(Client, "queries/get_client.sql", id,)
            .fetch_one(pool)
            .await?;

        Ok(client)
    }
}

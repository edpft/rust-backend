# Rust backend

```sh

```

## Setup

Create a new binary project and open VS Code

```sh
cargo new rust-backend
cd rust-backend/
code .
```

Initial git commit

```sh
git add .
git commit -m "initial commit"
git remote add origin https://github.com/edpft/rust-backend.git
git push -u origin main 
```

Add MIT license

```sh
wget https://raw.githubusercontent.com/git/git-scm.com/main/MIT-LICENSE.txt -O MIT-LICENSE
```

Add GitHub Rust `.gitignore`

```sh
wget https://raw.githubusercontent.com/github/gitignore/main/Rust.gitignore -O .gitignore
```

Install `cargo edit`

```sh
cargo install cargo-edit
```

Set up GitHub Actions workflows

Copy worflow templates from previous repo. These are based on the ones provided by [actions-rs](https://github.com/actions-rs)
and [Zero to Production in Rust](https://github.com/LukeMathWalker/zero-to-production). Though I've removed the `check` job as I now
think that's unnecessary.

```sh
wget -P .github/worflows https://raw.githubusercontent.com/edpft/rust-calendar-app/main/.github/workflows/general.yml
wget -P .github/worflows https://raw.githubusercontent.com/edpft/rust-calendar-app/main/.github/workflows/audit-on-push.yml
```

## Set up a health check endpoint

I want a `/health_check` that returns a `200` status code if the server is up and responding.

Install rocket

```sh
cargo add rocket@0.5.0-rc.2
```

Create a basic `main.rs` file which launches an empty rocket.

```rust
// main.rs
#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
}
```

Test that the `/health_check` endpoint returns an 200 status code
and no body.

```rust
// test.rs
use rocket::{local::blocking::Client, http::Status};

#[test]
fn test_health_check() {
    // Arrange
    let client = Client::tracked(super::rocket()).unwrap();

    // Act
    let response = client.get("/health_check").dispatch();

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_none());
}
```

Create a `routes.rs` file and add the `/health_check` endpoint 

```rust
// routes.rs
use rocket::http::Status;

#[get("/health_check")]
pub async fn health_check() -> Status {
    Status::Ok
}
```

Update the `main.rs` file:

```rust
// main.rs
#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;
mod routes;

use routes::health_check;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![health_check])
}
```

## Adding an add_client form

```sh
cargo add rocket_dyn_templates --features tera
```

```sh
wget -P templates https://raw.githubusercontent.com/SergioBenitez/Rocket/master/examples/forms/templates/macros.html.tera
```

## Adding Webpack to bundle CSS files

[Install NVM](https://github.com/nvm-sh/nvm#installing-and-updating)

```sh
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
```

Restart the terminal

```sh
bash
```

Install latest LTS version of Node.js

```sh
nvm install --lts
```

Activate Node

```sh
nvm use node
```

Initialize npm

```sh
npm init
```

Install `webpack` and `webpack-cli` locally

```sh
npm install webpack webpack-cli --save-dev
```

npm install --save-dev postcss-loader postcss postcss-preset-env

## Database

Based on: https://www.digitalocean.com/community/tutorials/how-to-install-postgresql-on-ubuntu-20-04-quickstart

### Install PostgreSQL

Update package index then upgrade system:

```sh
sudo apt update && sudo apt full-upgrade -y && sudo apt autoremove --purge -y && sudo apt autoclean
```

Install the Postgres package:

```sh
sudo apt install postgresql
```

Start the Postgres service:

```sh
sudo systemctl start postgresql.service
```

### Create a New Role

```sh
sudo -u postgres createuser --interactive
```
### Create a New Database

```sh
sudo -u postgres createdb rust-backend
```

### Add `rocket_db_pools` to dependencies

```sh
cargo add rocket_db_pools@0.1.0-rc.2 --features sqlx_postgres,sqlx_macros
```


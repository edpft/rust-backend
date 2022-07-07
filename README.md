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

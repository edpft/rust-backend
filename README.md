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
and [Zero to Production in Rust](https://github.com/LukeMathWalker/zero-to-production).

```sh
wget -P .github/worflows https://raw.githubusercontent.com/edpft/rust-calendar-app/main/.github/workflows/general.yml
wget -P .github/worflows https://raw.githubusercontent.com/edpft/rust-calendar-app/main/.github/workflows/audit-on-push.yml
```

## Set up a health check endpoint

I want a `/health_check` that returns a `200` status code if the server is up and responding. 

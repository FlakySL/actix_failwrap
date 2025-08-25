![`actix_failwrap` banner][banner]

[![Crates.io][crate-badge]][crate]
[![License][license-badge]][license]
[![Docs.rs][docs-badge]][docs]
[![Downloads][downloads-badge]][downloads]
[![Codecov][codecov-badge]][codecov]
[![tests][tests-badge]][codecov]
[![Discord][discord-badge]][discord]

`actix_failwrap` */ˈæk.tɪks ˈfeɪl.ræp/* ("*aktiks fail-rap*") A micro-package that enables ergonomic **error propagation** (via [`thiserror`][thiserror]) inside Actix Web route handlers.

This crate allows you to:

- ✅ Assign HTTP status codes to your [`thiserror`][thiserror] enums.
- 🧩 Customize the HTTP response with a builder function.
- ⚡ Use the [`?`][?] operator naturally inside route handlers.

---

## Table of Contents 📖

- [Features 🚀](#features-)
- [Installation 📦](#installation-)
- [Usage Example 🤔](#usage-example-)
- [Exported macros 🔧](#exported-macros-)
  - [`ErrorResponse` ⚙️](#deriveerrorresponse)
  - [`proof_route` ⚙️](#proof_route)
- [Security 🔐](#security-)
- [License 📜](#license-)

---

## Features 🚀

- ✅ **Automatic error-to-response conversion** using [`thiserror`][thiserror] enums
  Define route errors with `#[derive(ErrorResponse)]` to auto-generate `HttpResponse`.

- 🧩 **Custom response transformation per error enum**
  Use `#[transform_response(fn)]` to modify headers, body, or status codes.

- 🧠 **Per-variant status code overrides**
  Set status codes using `#[status_code(...)]` — supports both constants and numbers.

- 🔁 **Fallback behavior** for unannotated variants
  Variants without `#[status_code]` fall back to `#[default_status_code]` or HTTP 500.

- ✍️ **Extractor error mapping with `#[error_override(...)]`**
  Map deserialization or extractor failures to your own enum variant.

- ⚡ **Minimal boilerplate route macros with `#[proof_route(...)]`**
  Use `?` with error enums directly and skip [`actix_web`][actix-web] macro imports.

---

## Installation 📦

> [!IMPORTANT]
> The `actix_failwrap` macros rely on [`thiserror`][thiserror] for the `Display` implementation, and are tightly coupled with [`actix-web`][actix-web] for building HTTP responses.

This crate is published on [crates.io]
and is intended for use alongside [`actix-web`][actix-web]
and [`thiserror`][thiserror].

Add all three to your `Cargo.toml`:

```toml
[dependencies]
actix-web = "4"
thiserror = "1"
actix_failwrap = "1.0.0"
```

---

## Usage Example 🤔

This example shows a login route in Actix Web using `actix_failwrap`.

In your project you may have a module that declares models, in this case a `User` model.
In that file you may declare your [`thiserror`][thiserror] error that you may re-use for your handler.

```rust ignore
use serde::{Serialize, Deserialize};
use actix_failwrap::ErrorResponse;
use thiserror::Error;

// Custom error transformation function used by #[transform_response]
// Converts an error into a response with an "Error" header.
fn error_to_header(mut response: HttpResponseBuilder, error: String) -> HttpResponse {
  response.insert_header(("Error", error)).finish()
}

// Define a custom error enum for user-related errors.
#[derive(ErrorResponse, Error, Debug)]
// Default fallback for variants without #[status_code],
// if the attribute is not present, the default status code will be 500.
#[default_status_code(InternalServerError)]
// Function used to transform the final HttpResponse,
// if the attribute not present, the Display is mapped to the body.
#[transform_response(error_to_header)]
pub enum UserError {
  #[error("Either the email or the password is invalid. Please check the input credentials")]
  // This can also be a numeric HTTP status code.
  #[status_code(Unauthorized)]
  InvalidCredentials,

  #[error("Missing credentials, please, introduce your email and password.")]
  #[status_code(BadRequest)]
  MissingCredentials,
}

#[derive(Serialize, Deserialize)]
pub struct UserCredentials {
  pub email: String,
  pub password: String,
}

// Simulates a function that attempts to authenticate a user and returns a Result
pub fn obtain_user(credentials: UserCredentials) -> Result<User, UserError> {
  /* ... */
}
```

And another module that declares handlers, this example handler obtains a user
with some credentials and returns its JWT token if successful.

```rust ignore
use actix_failwrap::proof_route;
use actix_web::{web::Form, HttpResponse, HttpResponseBuilder};

use crate::models::user::{UserError, UserCredentials, obtain_user};

// Route macro expands to #[actix_web::post("/login")] and allows
// to use `Result<HttpResponse, _>`.
#[proof_route("POST /login")]
async fn post_login(
  // If the extractor (Form) fails, override it with MissingCredentials variant
  #[error_override(MissingCredentials)] credentials: Form<UserCredentials>
) -> Result<HttpResponse, UserError> {
  // Attempt to obtain the user; if it fails, propagate the error
  let user = obtain_user(credentials.into_inner())?;

  // On success, return a response with a "Login" header containing the JWT
  Ok(
    HttpResponse::Ok().
      .insert_header(("Login", user.jwt()))
      .finish()
  )
}
```

---

## Exported macros 🔧

This crate exports two macros: `ErrorResponse` and `proof_route`.

### `#[derive(ErrorResponse)]`

Implements `Into<actix_web::HttpResponse>` and `Into<actix_web::Error>` for your `thiserror` enums,
allowing direct propagation with the `?` operator in handlers.

> [!WARNING]
> Requires `#[derive(thiserror::Error)]` because it uses the `Display` implementation.

#### Supported Attributes

- `#[default_status_code(...)]`
  Fallback status code used if a variant does not have its own `#[status_code(...)]`
  Defaults to `InternalServerError` (500).

- `#[status_code(...)]`
  Sets the HTTP status code for a specific variant. Accepts a named status (e.g. `BadRequest`) or number (`400`).

- `#[transform_response(fn)]`
  Customizes how the response is built. Takes a function of signature:
  `fn(HttpResponseBuilder, String) -> HttpResponse`.

### `#[proof_route(...)]`

Simplifies route definition and error propagation.

```rust ignore
#[proof_route("POST /path")]
```

Expands to:

```rust ignore
#[actix_web::post("/path")]
```

An example function signature looks like

```rust ignore
#[proof_route("GET /users")]
async fn get_users() -> Result<HttpResponse, Error> {}
```

> [!TIP]
> You can use a `Result<T: actix_web::Responder, Error>` instead of `HttpResponse`.

Allows you to:

- Use `Result<HttpResponse, Error>` directly in route bodies.
- Avoid importing `#[post]`, `#[get]`, etc. individually.
- Support extractor error override via `#[error_override(...)]`.

---

## Security 🔐

Security is a top priority for us. If you believe you’ve found a security
vulnerability, **do not open a public issue**. Instead, please read
our [SECURITY.md](./SECURITY.md) policy and report it responsibly by
contacting us at [security@flaky.es].

---

## License 📜

This repository is dual licensed, If your repository is open source, the library is free of use, otherwise contact [licensing@flaky.es] for a custom license for your use case.

For more information read the [license file][license].

<!-- Reference Links -->
[?]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.try
[crates.io]: https://crates.io/crates/actix_failwrap
[actix-web]: https://crates.io/crates/actix-web
[thiserror]: https://crates.io/crates/thiserror

<!-- Contact information -->
[licensing@flaky.es]: mailto:licensing@flaky.es
[security@flaky.es]: mailto:security@flaky.es

<!-- Repository banner -->
[banner]: https://github.com/user-attachments/assets/15d5d3f2-3e78-49f2-8a09-b28b15bedd9f

<!-- Badge images -->
[crate-badge]: https://badges.ws/crates/v/actix_failwrap
[license-badge]: https://badges.ws/crates/l/actix_failwrap
[docs-badge]: https://badges.ws/crates/docs/actix_failwrap
[downloads-badge]: https://badges.ws/crates/dt/actix_failwrap
[codecov-badge]: https://img.shields.io/codecov/c/github/FlakySL/actix_failwrap
[tests-badge]: https://github.com/FlakySL/actix_failwrap/actions/workflows/020-tests.yml/badge.svg
[discord-badge]: https://badges.ws/discord/online/1344769456731197450

<!-- Badge targets -->
[crate]: https://crates.io/crates/actix_failwrap
[license]: https://github.com/FlakySL/actix_failwrap/blob/main/LICENSE
[docs]: https://docs.rs/actix_failwrap
[downloads]: https://docs.rs/actix_failwrap
[codecov]: https://app.codecov.io/gh/FlakySL/actix_failwrap
[discord]: https://discord.gg/AJWFyps23a

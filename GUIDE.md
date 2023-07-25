# Developer Guide

> Written 7/25/23.

Hi, welcome to Bubbel's backend!
This is a guide will introduce you to the architecture and layout of the project.

## Project Layout

Bubbel is designed as a majestic monolith that's split into two crates.

`bubbel-bath` contains all of the logic associated with the platform.
However, it doesn't do any networking (except for database operations).
Everything from emails to HTTP is outside of it's scope.
The reason why we did this is so that:
1. The platform much easier to test, no need for postman or other external
services.
2. Protocol / Library independence.
3. Scalability: in the far future, when we need horizontal scaling, we can
chop up multiple independent binaries with ease.

`bubbel-backend` essentially connects `bubbel-bath` to the internet and also
defines the conventions and types used by the front end.

## Type / Function Conventions

> PAY VERY CLOSE ATTENTION TO THESE CONVENTIONS!

### `bubbel-bath` API Conventions

All API input, output, and error types MUST be explicitly defined like so:

```rust
//  The input type.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DoSomething {
}

//  The output type.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DoSomethingOut {
}

//  The error type.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DoSomethingError {
    InvalidThing,
    Internal {
        ierror: String,
    },
}
```

Every API function like so:

```rust
fn do_something(req: DoSomething) -> Result<DoSomethingOut, DoSomethingError> {
    Ok(())
}
```

### `bubbel-backend` API Conventions

For the sake of consistency for the frontend, all input and output types look
like this:

```rust
#[derive(Serialize, Deserialize, JsonSchema)]
struct InDoSomething {
    #[serde(flatten)]
    pub req: DoSomething
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct ResDoSomething {
    error: Option<DoSomethingError>,
    #[serde(flatten)]
    pub res: Option<DoSomethingOut>
}
```

## `bubbel-bath` tests.

As a backend developer, your code has the ability to break everyone else's!
So please do yourself and everyone a favor by testing everything no matter how
simple the functionality.
You don't need one test for every API function as some tests can cover
multiple functions.
See [this file](/bubbel-bath/src/tests/README.md) for information on test
coverage.

## Adding an Endpoint

After developing an endpoint in `bubbel-bath` and testing it, you can finally
integrate this into the actual backend.

In `bubbel-backend` (`lib.rs`), add:

```rust
req!(InDoSomething, DoSomething);
res!(ResDoSomething, DoSomethingOut, DoSomethingError);
```

In `bubbel-backend` (`main.rs`), implement:

```rust
async fn api_delete_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InDeleteUser>,
) -> Json<ResDeleteUser> {
    //  Every endpoint works differently.
    todo!()
}
```

Then, add this function into the router in the `main` function.

In `bubbel-codegen`, add:

```rust
add!(
    "bubbelApiDoSomething",
    "/api/do_something",
    InDoSomething,
    ResDoSomething
);
```

## Pre-Push

Always run before pushing:

```sh
cargo fmt
cargo t
```

Also, if you added a data migration, remember to run the migration on the
production database.

```sh
diesel migration run --database-url "..."
```


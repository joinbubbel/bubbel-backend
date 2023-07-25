# Developer Tutorial

> Written 7/25/23.

Hi, welcome to Bubbel's backend!
This is a tutorial aimed at ~~sike~~ beginner backend developers.

## Introduction

First of all, I believe that the best way to understand a system is the feel
comfortable about it.
So let me just say, backend development is not as complicated as you think.

### What Even Is a "Backend"

I don't like the term "backend".
It sounds much fancier that what it really is.

At its core, a backend exposes *functions* that some client calls with some
*request* expecting some *response*.
There's not much else to it, and Bubbel's backend is designed to drive home
this point by decoupling networking and logic (I'll explain this next).

## Setup

See the setup section of [the guide](/GUIDE.md).

## The Parts

Bubbel's backend is make of multiple crates.
(Rust packages are called crates.)
The more detailed explanation can be found [in the guide](/GUIDE.md), but in
short, `bubbel-bath` holds the logic code and `bubbel-backend` connects
`bubbel-bath` to the internet.
It also defines the types used by the frontend.
`bubbel-codegen` and `bubbel-inspector` are side packages that help us automate
tedious work.

## State In the Backend

The backend has two ways of storing its state.
First, there's "state" which gets cleared on server restarts.
For example `AuthState` holds the user's sessions and keeps them logged in.
You can find all of the states in `bubbel-bath/src/state/`.

The other way is by storing data persistently on the database.
You would do this using `DataState` which holds a connection to the database.

## Tutorial Time!


### Business Logic

Ok, just for fun, let's implement `/api/dangerous_login` which logs a user in
as long as they give us the super secret code "cheese".

We need to go into `bubbel-bath/src/api/` to implement the logic.
Add a new file: `bubbel-bath/src/api/dangerous_login.rs`.
Each rust file is considered to be a module, so go to
`bubbel-bath/src/api/mod.rs` and add the new module.

```rust
mod dangerous_login;
```

Logic time!
First, we need to plan out what the inputs, outputs, and fail points of the API
call are.
By convention, we will define them explicitly like so:

```rust
use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DangerousLogin {
    pub user_id: UserId,
    pub super_secret_code: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DangerousLoginOut {
    pub token: String
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DangerousLoginError {
    UserNotFound,
    InvalidSuperSecretCode,
    Internal { ierror: String },
}
```

If you look into `bubbel-bath/src/api/`, almost every type looks exactly like
this.
This convention is maintained for the sake of consistency, and I expect you to do the
same.

Now, we need to decide what states we need access to in order to implement this.
Looks like we will need `DataState` to check if our user even exists, and
`AuthState` to log them in.

> Pro tip, run `cargo doc --open` and search for `State` to see all of the states.

```rust
fn dangerous_login(db: &mut DataState, auth: &mut AuthState, req: DangerousLogin) -> Result<DangerousLogin, DangerousLoginError> {
}
```

Once again, look into `bubbel-bath/src/api/`, and you will find that every API
function is written like this.
Anyway, let's implement the logic.

```rust
fn dangerous_login(db: &mut DataState, auth: &mut AuthState, req: DangerousLogin) -> Result<DangerousLogin, DangerousLoginError> {
    //  1. Does this user exist and are they verified?
    if let Some(user) = User::get(db, req.user_id)
        //  If we get an error from the database, throw an interal error.
        .map_err(|e| DangerousLogin::Internal {
            ierror: e.to_string(),
        })?
    {
        if !user.is_verified {
            //  The user exists but isn't verified.
            Err(DangerousLoginError::UserNotVerified)?;
        }
    } else {
        //  The user was `None`, so they don't exist.
        Err(DangerousLoginError::UserNotFound)?;
    }

    //  2. Did they get our secret code correct?
    if (req.super_secret_code != "cheese") {
        Err(DangerousLoginError::InvalidSuperSecretCode)?
    }

    //  3. All clear, they can log in!
    let token = auth.unchecked_auth_user(&req.user_id);
    Ok(DangerousLoginOut { token })
}
```

Notice that there are no raw database accesses here.
This is important!
Whenever you need data from the database, refrain from writing database
accesses in API functions.
Rather, move them to their respective types.
`User::get` is an example of this.

Finally, we need to export these types.
In `bubbel-bath/src/api/mod.rs`:

```rust
pub use dangerous_login::{dangerous_login, DangerousLogin, DangerousLoginOut, DangerousLoginError};
```

### Testing

Now, it's time to test your code!
As a backend developer, your code has the ability to break everyone else's, so
if you can test your code, test it!

Tests are always located in `bubbel-bath/src/tests/`.
As always, create `bubbel-bath/src/tests/test_dangerous_login.rs` add another
module into `bubbel-bath/src/tests/mod.rs`.

```
mod test_dangerous_login;
```

Now, let's implement the test.

```rust
use super::*;

#[test]
#[serial_test::serial]
pub fn test_dangerous_login() {
    //  1. Create the states we'll need for testing.
    let mut db = new_data_state();
    let mut auth = AuthState::default();
    let mut acc_limbo = AccountLimboState::default();
    
    //  2. Create a burner account.
    let acc = create_user(
        &mut db,
        CreateUser {
            email: "hello@gmail.com".to_owned(),
            username: "sikethedev".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .user_id;

    //  3. Verify the burner account.
    acc_limbo.push_user(acc1);
    acc_limbo.waive_user_verification(&mut db);

    //  4. Test!
    assert_eq!(
        dangerous_login(DangerousLogin {
            user_id: UserId(99999999),
            super_secret_code: "cheese".to_owned(),
        }),
        DangerousLoginError::UserNotFound
    );

    assert_eq!(
        dangerous_login(DangerousLogin {
            user_id: acc,
            super_secret_code: "cheezit".to_owned(),
        }),
        DangerousLoginError::InvalidSuperSecretCode
    );

    assert!(
        dangerous_login(DangerousLogin {
            user_id: acc,
            super_secret_code: "cheese".to_owned(),
        }).is_ok()
    );
}
```

Now, run your test with `cargo t`.
After a while, you should see all green.
If you get errors about the database, this means one of the following:

1. You have not set up the database.
2. Postgresql is not running

### Exposing Frontend Types

We have our logic, now we just need to expose our new types.
The convention for types can be found [in the guide](), but you don't actually
have to worry about it as our macros do the heavy lifting.
In `bubbel-backend/src/lib.rs` add the following:

```rust
req!(InDangerousLogin, DangerousLogin);
res!(ResDangerousLogin, DangerousLoginOut, DangerousLoginError);
```

Now, go to `bubbel-codegen/src/main.rs` and add the following:

```rust
add!(
    //  Frontend function.
    "bubbelApiDangerousLogin",
    //  Endpoint.
    "/api/dangerous_login",

    //  Our types.
    InDangerousLogin,
    ResDangerousLogin
);
```

### Exposing the API Function.

Go to `bubbel-backend/src/main.rs` and add a new HTTP post callback:

```rust
// -- snip --
    .route("/api/dangerous_login", post(api_dangerous_login))
// -- snip --

async fn api_dangerous_login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InDangerousLogin>,
) -> Json<ResDangerousLogin> {
    //  1. Log the incoming traffic.
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    //  2. Obtain access to certain states.
    let mut db = state.db.lock().unwrap();
    let mut auth = state.auth.write().unwrap();

    //  3. Call the API function.
    let res = match dangerous_login(&mut db, &mut auth, req.req) {
        Ok(res) => ResDangerousLogin {
            error: None,
            res: Some(res),
        },
        Err(e) => ResDangerousLogin {
            error: Some(e),
            res: Some(()),
        },
    };
    
    //  4. Log the outgoing traffic.
    debug.push_outgoing(&res);
    Json(res)
}

```

This function seems long, but really, when I implement one of these, I just copy
paste.

### Pre-Push

How, it's time to prepare for a push.
We didn't do any database migrations, so we don't need to worry about that.
However, we will need to generate code and notify frontend developers.

```sh
# Make sure that you are in the project root.
cargo r --bin bubbel-codegen.
```

You will now see changes in `sdks/`.
Make sure to update `sdks/README.md` and ping all the ~~victims~~ frontend
developers.

One more step: make sure that the rust code you push is quality by the
following commands one at a time.

```sh
cargo clippy
cargo fmt
cargo t
```

## Conclusion

Hey, congrats, you just implemented your first endpoint!
Of course, this was a very simple case and did not cover database migrations,
implementing your own states, etc.
If you want to know about how to do those things, ask David or refer to
[the guide](/GUIDE.md).
Good luck out there!


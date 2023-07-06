# SDKs

Currently, only Kotlin and Swift SDKs are provided.

## Reference

### Kotlin Notes

### Swift Notes

1. All member names will be in camel case, not snake case.
2. `BubbelError` may be thrown if a network error occurs.

### `DatabaseError`

```
type: |
    "NotFound"
    "UniqueViolation"
    "ForeignKeyViolation"
    "UnableToSendCommand"
    "SerializationFailure"
    "ReadOnlyTransaction"
    "NotNullViolation"
    "CheckViolation"
    "ClosedConnection"
    "Unknown"
uerror: MaybeNull<String>
```

1. `uerror` will have a value if `type` is `Unknown`.
2. Currently, you need to pay attention to `UniqueViolation` and `NotNullViolation`.
The rest are mostly internal errors that shouldn't ever occur.
3. The rest of these errors come directly
from [here](https://docs.rs/diesel/latest/diesel/result/enum.DatabaseErrorKind.html)
and [here](https://docs.rs/diesel/latest/diesel/result/enum.Error.html).

### `InCreateUser`

```
email: String
username: String
password: String
```

### `ResCreateUser`

```
error: MaybeNull<CreateUserError>
```

### `CreateUserError`

```
type: |
    "InvalidEmail"
    "InvalidUsername"
    "InvalidPassword"
    "InvalidPasswordCrypto"
    "DatabaseError"
dberror: MaybeNull<DatabaseError>
```

1. `dberror` will have a value if `type` is `"DatabaseError"`.
2. `"InvalidPasswordCrypto"` should pretty much never happen, but if it does, you can treat it as `"InvalidPassword"`.

### `InAuthUser`

```
username: String
password: String
```

### `ResAuthUser`

```
error: MaybeNull<AuthUserError>
token: String
username: String
email: String
```

### `AuthUserError`

```
type: |
    "InvalidCredentials"
    "InvalidPasswordCryto"
    "UserNotFound"
    "DatabaseError"
dberror: DatabaseError
```

1. `dberror` will have a value if `type` is `"DatabaseError"`.

### `InDeauthUser`

```
token: String
```

### `BUBBEL_BATH_DEV` / `bubbelBathDev`

This value represents the base url of Bubbel's API.
You will need this value for every SDK call.

Swift example:

```swift
bubbelApiAuthUser(bubbelBathDev, ...)
```

### `bubbelApiCreateUser` / `POST /api/create_user`

Create a new user account.
Uses `InCreateUser`, returning `ResCreateUser`.

### `bubbelApiAuthUser` / `POST /api/auth_user`

Authenticate a user, generating a new session token.
Uses `InAuthUser`, returning `ResAuthUser`

### `bubbelApiDeauthUser`

Invalidate a user's session token.
Uses `InDeauthUser`, returning nothing.

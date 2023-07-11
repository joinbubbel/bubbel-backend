# SDKs

The SDKs are in this directory.
Please these files.
Currently, only Kotlin, Swift, and Typescript SDKs are provided.

## Reference

### Kotlin Notes

1. Depends on OkHttp.

### Swift Notes

1. All member names will be in camel case, not snake case.
2. `BubbelError` may be thrown if a network error occurs.

### Typescript Notes

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
    "EmailOrPasswordTaken"
    "Internal"
ierror: MaybeNull<String>
```

1. `ierror` will have a value if `type` is `"Internal"`.
2. `"InvalidPasswordCrypto"` should pretty much never happen, but if it does, you can treat it as `"InvalidPassword"`.

### `InAuthUser`

```
username: String
password: String
```

### `ResAuthUser`

```
error: MaybeNull<AuthUserError>
token: MaybeNull<String>
username: MaybeNull<String>
email: MaybeNull<String>
```

### `AuthUserError`

```
type: |
    "InvalidCredentials"
    "InvalidPasswordCryto"
    "UserNotFound"
    "Internal"
ierror: MaybeNull<String>
```

1. `ierror` will have a value if `type` is `"Internal"`.

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

### `bubbelApiDeauthUser` / `POST /api/deauth_user`

Invalidate a user's session token.
Uses `InDeauthUser`, returning nothing.

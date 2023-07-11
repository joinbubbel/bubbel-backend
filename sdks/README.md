# SDKs

The SDKs are in this directory.
Please these files.
Currently, only Kotlin, Swift, and Typescript SDKs are provided.

TLDR: Every API call should look something like this:

```typescript
//  Typescript example.
//  bubbelBathDev refers to our dev instance, may vary in the future.
bubbelApiCreateUser(bubbelBathDev, {
    ...
});
```

## Reference

### Kotlin Notes

1. Depends on OkHttp.

### Swift Notes

1. All member names will be in camel case, not snake case.
2. `BubbelError` may be thrown if a network error occurs.

### Typescript Notes

1. None

### Bindings

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

### `bubbelApiCreateUser` / `POST /api/create_user`

Create a new user account.
Uses `InCreateUser`, returning `ResCreateUser`.

### `bubbelApiAuthUser` / `POST /api/auth_user`

Authenticate a user, generating a new session token.
Uses `InAuthUser`, returning `ResAuthUser`

### `bubbelApiDeauthUser` / `POST /api/deauth_user`

Invalidate a user's session token.
Uses `InDeauthUser`, returning nothing.

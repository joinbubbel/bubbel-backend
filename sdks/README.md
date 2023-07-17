# SDKs

The SDKs are in this directory.
Please these files.
Currently, only Kotlin, Swift, and Typescript SDKs are provided.

## Reference

### Kotlin Notes

1. Depends on OkHttp.
2. Must enable serialization with kotlinx.

### Swift Notes

1. All member names will be in camel case, not snake case.

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
user_id: MaybeNull<Integer>
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
    "UserNotVerified"
    "Internal"
ierror: MaybeNull<String>
```

1. `ierror` will have a value if `type` is `"Internal"`.

### `InDeauthUser`

```
token: String
```

### `ResDeauthUser`

```
error: Null
```

### `InVerifyAccount`

```
code: String
user_id: Integer
```

### `ResVerifyAccount`

```
error: MaybeNull<VerifyAccountError>
```

### `VerifyAccountError`

```
type: |
    "CodeTimedOutOrInvalidUser"
    "InvalidCode"
    "Internal"
ierror: MaybeNull<String>
```

1. `ierror` will have a value if `type` is `"Internal"`.

### `InSendVerify`

```
user_id: Integer
```

### `ResSendVerify`

```
error: MaybeNull<VerifyAccountError>
```

### `SendVerifyError`

```
type: |
    "SendVerification"
    "ResendTooSoon"
    "Internal"
ierror: MaybeNull<String>
```

1. `ierror` will have a value if `type` is `"Internal"`.

### `InSetUserProfile`

```
token: String
name: String
display_name: String
description: String
pfp: String
banner: String
```

### `ResSetUserProfile`

```
error: MaybeNull<SetUserProfileError>
```

### `SetUserProfileError`

```
type: |
    "NoAuth"
    "Internal"
ierror: MaybeNull<String>
```

1. `ierror` will have a value if `type` is `"Internal"`.

### `InDeleteUser`

```
token: String
```

### `ResDeleteUser`

```
error: MaybeNull<DeleteUserError>
```

### `DeleteUserError`

```
type: |
    "NoAuth"
    "Internal"
ierror: MaybeNull<String>
```

1. `ierror` will have a value if `type` is `"Internal"`.

### `bubbelApiCreateUser` / `POST /api/create_user`

Create a new user account.
This will not send a verification code.
Uses `InCreateUser`, returning `ResCreateUser`.

### `bubbelApiAuthUser` / `POST /api/auth_user`

Authenticate a user, generating a new session token.
Uses `InAuthUser`, returning `ResAuthUser`

### `bubbelApiDeauthUser` / `POST /api/deauth_user`

Invalidate a user's session token.
Uses `InDeauthUser`, returning nothing.

### `bubbelApiVerifyAccount` / `POST /api/verify_account`

Verify a user's account using a user ID and code.

### `bubbelApiSendVerify` / `POST /api/send_verify`

Send a verification code to the user to verify their account.

### `bubbelApiSetUserProfile` / `POST /api/set_user_profile`

Set the user's profile data.
A `null` field has no effect on the currently stored data.

### `bubbelApiDeleteUser` / `POST /api/set_user_profile`

Delete a user's account, logging them out and deleting all of their data.

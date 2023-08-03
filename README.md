# Bubbel Backend

Hello!
Welcome to Bubbel's backend repository.
This codebase contains most of the backend's logic as well as the actual runtime.

## SDK Users

Bindings to the backend are found in [`sdks/`](./sdks/)
Details and documentation can be found there.
You can also have a look at the running backend's docs
[here](https://api.joinbubbel.com/docs/bubbel_backend/).

## Running

You can run the backend with the following script.

```sh
#!/bin/sh

set -ex

export BUBBEL_ACCOUNT_VERIFICATION_FROM_EMAIL="..."
export BUBBEL_ACCOUNT_VERIFICATION_FROM_EMAIL_PASSWORD="..."
export BUBBEL_DATABASE_URL="..."
export BUBBEL_DEBUG_INSPECTOR_PASSWORD="..."
export BUBBEL_ENABLE_DEBUG_INSPECTOR="this can be anything"
export BUBBEL_ENABLE_WAIVE_ALL_ACCOUNT_VERIFICATION="this can be anything"
export BUBBEL_USER_SALT="..."
export BUBBEL_TLS_CERT_PATH="..."
export BUBBEL_TLS_KEY_PATH="..."
export BUBBEL_DOCS_PATH="..."

cargo r
```

Note that you should almost never be doing this manually.
Deployment scripts are available (just not publicly).
Additionally, testing should be done in the form of unit tests.

## Contributors

Contributors should read the [guide](./GUIDE.md)
as well as the [tutorial](./TUTORIAL.md).

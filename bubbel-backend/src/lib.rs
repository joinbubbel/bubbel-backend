//! All types here follow the same convention:
//!
//! ```rust,ignore
//! #[derive(Serialize, Deserialize)]
//! struct InFooBar {
//!     #[serde(flatten)]
//!     pub req: FooBar
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct ResFooBar {
//!     error: Option<FooBarError>,
//!     #[serde(flatten)]
//!     pub res: Option<FooBarOut>
//! }
//! ```
//!
//! This makes serialization easy and consistent.
//!

pub use bubbel_bath::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InCreateUser {
    #[serde(flatten)]
    pub req: CreateUser,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ResCreateUser {
    pub error: Option<CreateUserError>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InAuthUser {
    #[serde(flatten)]
    pub req: AuthUser,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ResAuthUser {
    pub error: Option<AuthUserError>,
    #[serde(flatten)]
    pub res: Option<AuthUserOut>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InDeauthUser {
    #[serde(flatten)]
    pub req: DeauthUser,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ResDeauthUser {
    pub error: Option<()>,
}

//! All types here follow the same convention:
//!
//! ```
//! struct InFooBar {
//!     #[serde(flatten)]
//!     pub req: FooBar
//! }
//!
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

#[derive(Serialize, Deserialize)]
pub struct InCreateUser {
    #[serde(flatten)]
    pub req: CreateUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResCreateUser {
    pub error: Option<CreateUserError>,
}

#[derive(Serialize, Deserialize)]
pub struct InAuthUser {
    #[serde(flatten)]
    pub req: AuthUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResAuthUser {
    pub error: Option<AuthUserError>,
    #[serde(flatten)]
    pub res: Option<AuthUserOut>,
}

#[derive(Serialize, Deserialize)]
pub struct InDeauthUser {
    #[serde(flatten)]
    pub req: DeauthUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResDeauthUser {
    pub error: Option<()>,
}

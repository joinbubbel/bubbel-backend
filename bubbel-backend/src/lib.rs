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
//!     pub res: Option<FooBarOut>
//! }
//! ```
//!
//! This makes serialization easy and consistent.
//!

pub use bubbel_bath::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

type Empty = ();

macro_rules! req {
    ($IN: ident, $REQIN: ident) => {
        #[derive(Serialize, Deserialize, JsonSchema)]
        pub struct $IN {
            #[serde(flatten)]
            pub req: $REQIN,
        }
    };
}

macro_rules! res {
    ($RES: ident, $RESOUT: ident, $RESERROR: ident) => {
        #[derive(Serialize, Deserialize, JsonSchema)]
        pub struct $RES {
            pub error: Option<$RESERROR>,
            pub res: Option<$RESOUT>,
        }
    };
}

req!(InCreateUser, CreateUser);
res!(ResCreateUser, CreateUserOut, CreateUserError);

req!(InAuthUser, AuthUser);
res!(ResAuthUser, AuthUserOut, AuthUserError);

req!(InDeauthUser, DeauthUser);
res!(ResDeauthUser, Empty, Empty);

req!(InVerifyAccount, VerifyAccount);
res!(ResVerifyAccount, Empty, VerifyAccountError);

req!(InSendVerify, SendVerify);
res!(ResSendVerify, Empty, SendVerifyError);

req!(InSetUserProfile, SetUserProfile);
res!(ResSetUserProfile, Empty, SetUserProfileError);

req!(InGetUserProfile, GetUserProfile);
res!(ResGetUserProfile, GetUserProfileOut, GetUserProfileError);

req!(InDeleteUser, DeleteUser);
res!(ResDeleteUser, Empty, DeleteUserError);

req!(InCreateClub, CreateClub);
res!(ResCreateClub, CreateClubOut, CreateClubError);

req!(InGetClubProfile, GetClubProfile);
res!(ResGetClubProfile, GetClubProfileOut, GetClubProfileError);

req!(InSetClubProfile, SetClubProfile);
res!(ResSetClubProfile, SetClubProfileOut, SetClubProfileError);

req!(InDeleteClub, DeleteClub);
res!(ResDeleteClub, Empty, DeleteClubError);

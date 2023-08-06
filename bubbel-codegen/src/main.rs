use bubbel_backend::*;
use json::{object, JsonValue};
use project_root::get_project_root;
use schemars::schema_for;

mod kotlin;
mod swift;
mod typescript;

type GetArgs = fn() -> Vec<&'static str>;
type PostProcess = fn(String) -> String;
type GetFetch = fn(&Endpoint) -> String;

pub struct Endpoint {
    fn_name: &'static str,
    endpoint: &'static str,
    in_ty: &'static str,
    out_ty: &'static str,
}

struct Schema {
    current_num: usize,
    final_val: JsonValue,
}

impl Schema {
    pub fn push_schema_str(&mut self, s: &str) {
        let val = json::parse(s).unwrap();
        let title = val["title"].as_str().unwrap();
        self.final_val["definitions"][title] = val.clone();
        self.final_val["properties"][format!("t{}", self.current_num)] = object! {
            "$ref": format!("#/definitions/{}", title)
        };
        let JsonValue::Object(val) = val else {
            unreachable!()
        };
        if let Some(definitions) = val.get("definitions") {
            for (k, v) in definitions.entries() {
                self.final_val["definitions"][k] = v.clone();
            }
        }
        self.current_num += 1;
    }
}

fn invoke_quicktype(get_args: GetArgs, post_process: PostProcess, schema: &str) -> String {
    let temp_dir_path = std::env::temp_dir();
    let temp_dir = temp_dir_path.to_str().unwrap();
    let schema_dir = format!("{}/bubbel_codegen_schema.json", &temp_dir);
    let out_dir = format!("{}/bubbel_codegen_out", &temp_dir);

    std::fs::write(&schema_dir, schema).unwrap();

    let mut args = vec![
        "quicktype",
        "-o",
        &out_dir,
        "--src-lang",
        "schema",
        &schema_dir,
    ];
    args.append(&mut get_args());

    std::process::Command::new("npx")
        .args(args.iter())
        .output()
        .unwrap();

    post_process(std::fs::read_to_string(out_dir).unwrap())
}

fn main() {
    let mut schema = Schema {
        current_num: 0,
        final_val: object! {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "definitions": {},
            "properties": {},
        },
    };
    let mut endpoints = vec![];

    macro_rules! add {
        ($FN_NAME: expr, $ENDPOINT: expr, $IN: ty, $OUT: ty) => {
            schema.push_schema_str(&serde_json::to_string(&schema_for!($IN)).unwrap());
            schema.push_schema_str(&serde_json::to_string(&schema_for!($OUT)).unwrap());
            endpoints.push(Endpoint {
                fn_name: $FN_NAME,
                endpoint: $ENDPOINT,
                in_ty: stringify!($IN),
                out_ty: stringify!($OUT),
            });
        };
    }

    add!(
        "bubbelApiCreateUser",
        "/api/create_user",
        InCreateUser,
        ResCreateUser
    );

    add!(
        "bubbelApiAuthUser",
        "/api/auth_user",
        InAuthUser,
        ResAuthUser
    );

    add!(
        "bubbelApiDeauthUser",
        "/api/deauth_user",
        InDeauthUser,
        ResDeauthUser
    );

    add!(
        "bubbelApiVerifyAccount",
        "/api/verify_account",
        InVerifyAccount,
        ResVerifyAccount
    );

    add!(
        "bubbelApiSendVerify",
        "/api/send_verify",
        InSendVerify,
        ResSendVerify
    );

    add!(
        "bubbelApiSetUserProfile",
        "/api/set_user_profile",
        InSetUserProfile,
        ResSetUserProfile
    );

    add!(
        "bubbelApiGetUserProfile",
        "/api/get_user_profile",
        InGetUserProfile,
        ResGetUserProfile
    );

    add!(
        "bubbelApiDeleteUser",
        "/api/delete_user",
        InDeleteUser,
        ResDeleteUser
    );

    add!(
        "bubbelApiCreateClub",
        "/api/create_club",
        InCreateClub,
        ResCreateClub
    );

    add!(
        "bubbelApiGetClubProfile",
        "/api/get_club_profile",
        InGetClubProfile,
        ResGetClubProfile
    );

    add!(
        "bubbelApiSetClubProfile",
        "/api/set_club_profile",
        InSetClubProfile,
        ResSetClubProfile
    );

    add!(
        "bubbelApiDeleteClub",
        "/api/delete_club",
        InDeleteClub,
        ResDeleteClub
    );

    add!(
        "bubbelApiGetUserProfileWithUsername",
        "/api/get_user_profile_with_username",
        InGetUserProfileWithUsername,
        ResGetUserProfileWithUsername
    );

    add!(
        "bubbelApiAddFriendConnection",
        "/api/add_friend_connection",
        InAddFriendConnection,
        ResAddFriendConnection
    );

    add!(
        "bubbelApiGetFriendConnections",
        "/api/get_friend_connections",
        InGetFriendConnections,
        ResGetFriendConnections
    );

    add!(
        "bubbelApiJoinClub",
        "/api/join_club",
        InJoinClub,
        ResJoinClub
    );

    add!(
        "bubbelApiUnjoinClub",
        "/api/unjoin_club",
        InUnjoinClub,
        ResUnjoinClub
    );

    add!(
        "bubbelApiGetClubMembers",
        "/api/get_club_members",
        InGetClubMembers,
        ResGetClubMembers
    );

    add!(
        "bubbelApiGetUserClubs",
        "/api/get_user_clubs",
        InGetUserClubs,
        ResGetUserClubs
    );

    let project_root = get_project_root().unwrap();

    let mut ts_out = project_root.clone();
    ts_out.push("sdks/bubbelBackend.ts");

    let mut swift_out = project_root.clone();
    swift_out.push("sdks/BubbelBackend.swift");

    let mut kotlin_out = project_root.clone();
    kotlin_out.push("sdks/BubbelBackend.kt");

    let targets = [
        (
            ts_out,
            typescript::get_args as GetArgs,
            typescript::post_process as PostProcess,
            typescript::get_fetch as GetFetch,
        ),
        (
            swift_out,
            swift::get_args as GetArgs,
            swift::post_process as PostProcess,
            swift::get_fetch as GetFetch,
        ),
        (
            kotlin_out,
            kotlin::get_args as GetArgs,
            kotlin::post_process as PostProcess,
            kotlin::get_fetch as GetFetch,
        ),
    ];

    for (out_dir, get_args, post_process, get_fetch) in targets {
        let mut out = invoke_quicktype(get_args, post_process, &schema.final_val.to_string());
        for endpoint in endpoints.iter() {
            out.push('\n');
            out += &get_fetch(endpoint);
        }
        std::fs::write(out_dir, out).unwrap();
    }
}

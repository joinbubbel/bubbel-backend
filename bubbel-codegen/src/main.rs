use bubbel_backend::*;
use schemajen::{accumulator_choose_with_str, codegen, generate, TypeAccumulator};

fn gen_ty<T: Serialize>(mut acc: Box<dyn TypeAccumulator>, name: &str, sample: T) {
    let json = serde_json::to_string(&sample).unwrap();
    println!("{}\n", generate(acc.as_mut(), name, &json).unwrap());
}

fn new_acc(lang: &str) -> Box<dyn TypeAccumulator> {
    accumulator_choose_with_str(lang).unwrap_or_else(|| {
        panic!(
            "Got bad codegen language, could be {:?}.",
            codegen::ACCUMULATOR_SUPPORT_LIST
        )
    })
}

fn main() {
    let Some(lang) = std::env::args().nth(1) else {
        panic!("Expected codegen language, could be: {:?}.", codegen::ACCUMULATOR_SUPPORT_LIST)
    };

    //  InCreateUser
    gen_ty(
        new_acc(&lang),
        "InCreateUser",
        InCreateUser {
            req: CreateUser {
                email: "".to_owned(),
                username: "".to_owned(),
                password: "".to_owned(),
            },
        },
    );

    //  ResCreateUser
    gen_ty(
        new_acc(&lang),
        "ResCreateUser",
        ResCreateUser {
            error: Some(CreateUserError::DatabaseError {
                dberror: DatabaseError::Unknown {
                    uerror: "".to_string(),
                },
            }),
        },
    );

    //  InAuthUser
    gen_ty(
        new_acc(&lang),
        "InAuthUser",
        InAuthUser {
            req: AuthUser {
                username: "".to_owned(),
                password: "".to_owned(),
            },
        },
    );

    //  ResAuthUser
    gen_ty(
        new_acc(&lang),
        "ResAuthUser",
        ResAuthUser {
            error: Some(AuthUserError::DatabaseError {
                dberror: DatabaseError::Unknown {
                    uerror: "".to_owned(),
                },
            }),
            res: Some(AuthUserOut {
                token: UserToken("".to_owned()),
                username: "".to_owned(),
                email: "".to_owned(),
            }),
        },
    );

    //  InDeauthUser
    gen_ty(
        new_acc(&lang),
        "InDeauthUser",
        InDeauthUser {
            req: DeauthUser {
                token: UserToken("".to_owned()),
            },
        },
    );

    //  ResDeauthUser
    gen_ty(
        new_acc(&lang),
        "ResDeauthUser",
        ResDeauthUser { error: Some(()) },
    );
}

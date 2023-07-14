use bubbel_backend::*;
use colored_json::prelude::*;
use schemars::schema_for;
use serde_json::to_string_pretty;

macro_rules! print {
    ($T: ty) => {
        println!("\n<=====================>\n");
        println!(
            "{}",
            to_string_pretty(&schema_for!($T))
                .unwrap()
                .to_colored_json_auto()
                .unwrap()
        );
    };
}

fn main() {
    print!(InCreateUser);
    print!(CreateUserError);
    print!(ResCreateUser);

    print!(InAuthUser);
    print!(AuthUserError);
    print!(ResAuthUser);

    print!(InDeauthUser);
    print!(ResDeauthUser);
}

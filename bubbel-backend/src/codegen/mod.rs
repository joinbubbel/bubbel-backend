use json::{object, JsonValue};
use project_root::get_project_root;
pub use schemars::schema_for;

mod custom_kotlin;
mod swift;
mod typescript;

type GetArgs = fn() -> Vec<&'static str>;
type PostProcess = fn(String) -> String;
type GetFetch = fn(&Endpoint) -> String;

macro_rules! add_codegen_endpoint {
    ($CTX: expr, $FN_NAME: expr, $ENDPOINT: expr, $IN: ty, $OUT: ty) => {
        $CTX.schema
            .push_schema_str(&serde_json::to_string(&schema_for!($IN)).unwrap());
        $CTX.schema
            .push_schema_str(&serde_json::to_string(&schema_for!($OUT)).unwrap());
        $CTX.endpoints.push(Endpoint {
            fn_name: $FN_NAME,
            endpoint: $ENDPOINT,
            in_ty: stringify!($IN),
            out_ty: stringify!($OUT),
        });
    };
}

pub struct CodegenContext {
    pub schema: Schema,
    pub endpoints: Vec<Endpoint>,
}

impl CodegenContext {
    pub fn new() -> Self {
        let schema = Schema {
            current_num: 0,
            final_val: object! {
                "$schema": "http://json-schema.org/draft-07/schema#",
                "type": "object",
                "definitions": {},
                "properties": {},
            },
            raw_schemas: vec![],
        };
        let endpoints = vec![];
        CodegenContext { schema, endpoints }
    }

    pub fn gen_and_write(&self) {
        let project_root = std::path::PathBuf::from(option_env!("BUBBEL_CODEGEN").unwrap());

        let mut ts_out = project_root.clone();
        ts_out.push("bubbelBackend.ts");

        let mut swift_out = project_root.clone();
        swift_out.push("BubbelBackend.swift");

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
        ];

        for (out_dir, get_args, post_process, get_fetch) in targets {
            let mut out =
                invoke_quicktype(get_args, post_process, &self.schema.final_val.to_string());
            for endpoint in self.endpoints.iter() {
                out.push('\n');
                out += &get_fetch(endpoint);
            }
            std::fs::write(out_dir, out).unwrap();
        }

        custom_kotlin::gen_from_schema(project_root.clone(), self);
    }
}

pub struct Endpoint {
    pub fn_name: &'static str,
    pub endpoint: &'static str,
    pub in_ty: &'static str,
    pub out_ty: &'static str,
}

pub struct Schema {
    current_num: usize,
    final_val: JsonValue,
    raw_schemas: Vec<(String, String)>,
}

impl Schema {
    pub fn push_schema_str(&mut self, s: &str) {
        let val = json::parse(s).unwrap();
        let title = val["title"].as_str().unwrap();

        self.raw_schemas.push((title.to_owned(), s.to_owned()));

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

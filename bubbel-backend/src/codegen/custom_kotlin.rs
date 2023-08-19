use super::*;

pub fn gen_from_schema(project_root: std::path::PathBuf, context: &CodegenContext) {
    let temp_dir_path = std::env::temp_dir();
    let temp_dir = temp_dir_path.to_str().unwrap().to_owned();

    let mut joins = vec![];
    for (title, schema) in context.schema.raw_schemas.iter().cloned() {
        let temp_dir = temp_dir.clone();
        let project_root = project_root.clone();
        joins.push(std::thread::spawn(move || {
            let schema_dir = format!("{}/bubbel_codegen_schema_{}.json", &temp_dir, title);
            let out_dir = format!("{}/kotlin/{}.kt", &project_root.to_str().unwrap(), title);

            std::fs::write(&schema_dir, schema).unwrap();
            let args = vec![
                "quicktype",
                "-o",
                &out_dir,
                "--src-lang",
                "schema",
                &schema_dir,
                "-l",
                "kotlin",
                "--framework",
                "kotlinx",
                "--package",
                "com.example.bubbel.model.backend",
            ];

            std::process::Command::new("npx")
                .args(args.iter())
                .output()
                .unwrap();

            let out = std::fs::read_to_string(&out_dir).unwrap();
            let out = post_process_types(&out);
            std::fs::write(&out_dir, out).unwrap();

            out_dir.clone()
        }));
    }

    let mut out_dirs = vec![];
    for join in joins.into_iter() {
        out_dirs.push(std::path::PathBuf::from(join.join().unwrap()));
    }

    for out_dir in out_dirs.iter() {
        let stem_str = out_dir.file_stem().unwrap().to_str().unwrap();
        if &stem_str[0..=2] == "Res" {
            let mut input = std::fs::read_to_string(out_dir).unwrap();
            input = input.replace(" Type(", &format!(" {}ErrorType(", stem_str));
            let output = input.replace(" Type,", &format!(" {}ErrorType,", stem_str));
            std::fs::write(out_dir, output).unwrap();
        }
    }

    let mut out_buf = String::new();

    out_buf += r#"package com.example.bubbel.repository

import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import com.example.bubbel.model.backend.*
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response
import retrofit2.http.Body
import retrofit2.http.POST

interface backendService {"#;

    for endpoint in context.endpoints.iter() {
        out_buf += &get_service(endpoint);
    }

    out_buf += r#"
}

//  This was originally went in "RetrofitClient.kt"
object RetrofitClient {
    val api: backendService by lazy {
        Retrofit.Builder()
            .baseUrl("https://api.joinbubbel.com")
            .addConverterFactory(GsonConverterFactory.create())
            .build()
            .create(backendService::class.java)
    }
}

//  This was originally went in "XXXRepository.kt"
class BackendRepository {
    private val backendService = RetrofitClient.api
"#;

    for endpoint in context.endpoints.iter() {
        out_buf += &get_repo(endpoint);
    }

    out_buf += "\n}";

    let out_dir = format!(
        "{}/kotlin/backendRepository.kt",
        &project_root.to_str().unwrap(),
    );

    std::fs::write(out_dir, out_buf).unwrap();
}

fn get_short_fn_name(fn_name: &str) -> String {
    fn_name
        .replace("bubbelApi", "")
        .chars()
        .enumerate()
        .map(|(idx, c)| {
            if idx == 0 {
                c.to_lowercase().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}

fn get_service(e: &Endpoint) -> String {
    let short_fn_name = get_short_fn_name(e.fn_name);
    format!(
        r#"
    @POST("{}")
    fun {}(@Body userData: {}): Call<{}>"#,
        e.endpoint, short_fn_name, e.in_ty, e.out_ty
    )
}

fn get_repo(e: &Endpoint) -> String {
    let short_fn_name = get_short_fn_name(e.fn_name);
    format!(
        r#"
    suspend fun {}(request: {},  onSuccess: ({}?) -> Unit, onError: (String) -> Unit){{
        backendService.{}(request).enqueue(object : Callback<{}> {{
            override fun onResponse(call: Call<{}>, response: Response<{}>) {{
                if (response.isSuccessful) {{
                    val out: {}? = response.body()
                    onSuccess(out)
                }} else {{
                    onError(response.errorBody()?.string() ?: "Unknown error occurred")
                }}
            }}

            override fun onFailure(call: Call<{}>, t: Throwable) {{
                onError(t.message ?: "Network request failed")
            }}
        }})
    }}"#,
        short_fn_name,
        e.in_ty,
        e.out_ty,
        short_fn_name,
        e.out_ty,
        e.out_ty,
        e.out_ty,
        e.out_ty,
        e.out_ty
    )
}

fn post_process_types(s: &str) -> String {
    //  GSON support.
    s.replace("SerialName", "SerializedName")
}

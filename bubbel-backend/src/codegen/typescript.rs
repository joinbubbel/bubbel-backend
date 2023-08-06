use super::*;

pub fn get_args() -> Vec<&'static str> {
    vec!["-l", "typescript", "--just-types"]
}

pub fn post_process(s: String) -> String {
    format!("const bubbelBathDev = 'https://api.joinbubbel.com';{}", s)
}

pub fn get_fetch(e: &Endpoint) -> String {
    format!(
        "export async function {}(req: {}): Promise<{}> {{
            let fetchRes = await fetch(bubbelBathDev + '{}', {{
                method: 'post',
                headers: {{
                    'Content-Type': 'application/json',
                }},

                body: JSON.stringify(req),
            }});
            let resText = await fetchRes.text();
            return JSON.parse(resText);
        }}",
        e.fn_name, e.in_ty, e.out_ty, e.endpoint
    )
}

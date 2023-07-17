pub fn get_args() -> Vec<&'static str> {
    vec!["-l", "kotlin", "--framework", "kotlinx", "--package", "a"]
}

pub fn post_process(s: String) -> String {
    format!(
        "val BUBBEL_BATH_DEV = 'https://bubbel-bath.onrender.com'\n{}",
        s.replace("package a", "")
    )
}

pub fn get_fetch(_: &str, _: &str, _: &str, _: &str) -> String {
    todo!()
}

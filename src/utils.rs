use std::env;

pub fn base_path() -> String {
    env::var("BASE_PATH").unwrap_or(env::var("CARGO_MANIFEST_DIR").unwrap_or("./".to_owned()))
}

pub fn static_path() -> String {
    let mut str = base_path();
    str.push_str("/static/");

    str
}

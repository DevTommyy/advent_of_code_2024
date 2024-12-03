// utils

use std::env;

pub fn load_input() -> String {
    let package_name = env::var("CARGO_PKG_NAME").unwrap_or_default();

    let input_file_path = format!("./{}/input.txt", package_name);

    input_file_path
}

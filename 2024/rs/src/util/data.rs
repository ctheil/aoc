use std::fs;

pub fn get_data_from_file(file_name: &str) -> String {
    let dir = std::env::current_dir()
        .unwrap()
        .to_str()
        .expect("Failed to get current directory.")
        .to_owned();

    let path = format!("{dir}/data/{file_name}");

    fs::read_to_string(path).expect("Failed to read the file...")
}

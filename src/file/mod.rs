use std::fs;
use std::path::Path;
pub fn read_file(module: &str) -> String {
    let file_path = format!("src/{}/input.txt", module.split("::").last().unwrap()).to_owned();
    let path = Path::new(file_path.as_str());
    fs::read_to_string(path).expect("Should have been able to read the file")
}

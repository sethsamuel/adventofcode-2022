// mod day1;
// mod day2;
mod day3;

use std::fs;
use std::path::Path;
fn read_file(module: &str) -> String {
    let file_path = format!("src/{}/input.txt", module.split("::").last().unwrap()).to_owned();
    let path = Path::new(file_path.as_str());
    fs::read_to_string(path).expect("Should have been able to read the file")
}

fn main() {
    // day1::part1();
    // day1::part2();
    // day2::part1();
    // day2::part2();
    // day3::part1();
    day3::part2();
}

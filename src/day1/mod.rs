use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct Elf {
    total_weight: u64,
}

fn find_highest_weight(elfs: &mut Vec<Elf>) -> Elf {
    elfs.sort_by_key(|e| e.total_weight);
    elfs.last().unwrap().to_owned()
}

fn parse_file(text: &str) -> Vec<Elf> {
    let elf_lines = text.split("\n\n");
    let elfs = elf_lines
        .map(|lines| {
            let weights: Vec<u64> = lines
                .split("\n")
                .map(|line| line.trim().parse::<u64>().unwrap())
                .collect();
            Elf {
                total_weight: weights.into_iter().sum(),
            }
        })
        .collect();

    elfs
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_parse_file() {
        let elfs = parse_file(TEST_STR);
        assert_eq!(elfs.len(), 5);
        assert_eq!(elfs.first().unwrap().total_weight, 1000 + 2000 + 3000);
    }

    #[test]
    fn test_find_highest_weight() {
        let mut elfs = parse_file(TEST_STR);
        let elf = find_highest_weight(&mut elfs);
        assert_eq!(elf.total_weight, 24_000);
    }
}

pub fn part1() {
    let path = Path::new("src/day1/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let elf = find_highest_weight(&mut parse_file(contents.as_str()));
    println!("{}", elf.total_weight);
}

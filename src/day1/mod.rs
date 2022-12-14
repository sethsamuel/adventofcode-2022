use crate::file::read_file;

#[derive(Debug, Clone)]
struct Elf {
    total_weight: u64,
}

fn find_highest_weight(elfs: &mut Vec<Elf>) -> Elf {
    elfs.sort_by_key(|e| e.total_weight);
    elfs.last().unwrap().to_owned()
}

fn find_top_3_weight(elfs: &mut Vec<Elf>) -> u64 {
    elfs.sort_by_key(|e| e.total_weight);
    elfs.iter().rev().take(3).map(|e| e.total_weight).sum()
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

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let elf = find_highest_weight(&mut parse_file(contents.as_str()));
    println!("{}", elf.total_weight);
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());

    println!("{}", find_top_3_weight(&mut parse_file(contents.as_str())));
}

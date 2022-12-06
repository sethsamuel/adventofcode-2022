use std::collections::HashSet;

use crate::file::read_file;

fn parse_file(text: &str) -> &str {
    text
}

fn find_first_unique_index(size: usize, text: &str) -> u32 {
    for i in size..text.len() {
        let slice = &text[i - size..i];
        let set = slice.chars().collect::<HashSet<_>>();
        if set.len() == size {
            return i as u32;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_find_first_unique_index() {
        let input = parse_file(TEST_STR);
        assert_eq!(find_first_unique_index(4, input), 7);
        assert_eq!(
            find_first_unique_index(4, "bvwbjplbgvbhsrlpgdmjqwftvncz"),
            5
        );
        assert_eq!(
            find_first_unique_index(4, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            11
        );

        assert_eq!(find_first_unique_index(14, input), 19);
        assert_eq!(
            find_first_unique_index(14, "bvwbjplbgvbhsrlpgdmjqwftvncz"),
            23
        );
        assert_eq!(
            find_first_unique_index(14, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            26
        );
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let input = parse_file(&contents);
    println!("{:?}", find_first_unique_index(4, input));
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let input = parse_file(&contents);
    println!("{:?}", find_first_unique_index(14, input));
}

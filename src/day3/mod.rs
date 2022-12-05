use std::collections::HashSet;

type Rucksack = (Vec<char>, Vec<char>);

fn parse_file(text: &str) -> Vec<Rucksack> {
    let lines = text.lines();
    lines
        .map(|l| {
            let chars = l.chars();
            let split = chars.clone().count() / 2;
            (
                chars.clone().take(split).collect::<Vec<char>>(),
                chars.skip(split).collect::<Vec<char>>(),
            )
        })
        .collect()
}

fn find_overlap(rucksack: &Rucksack) -> char {
    let left = rucksack.0.iter().collect::<HashSet<_>>();
    let right = rucksack.1.iter().collect::<HashSet<_>>();
    let inter = left.intersection(&right).collect::<Vec<_>>();
    let c = inter.first().unwrap();
    ***c
}

fn find_overlaps(rucksacks: &Vec<Rucksack>) -> Vec<char> {
    rucksacks.iter().map(|r| find_overlap(r)).collect()
}

fn sum_overlaps(overlaps: &Vec<char>) -> u32 {
    let values = overlaps.iter().map(|c| {
        if c.is_uppercase() {
            *c as u8 - 38
        } else {
            *c as u8 - 96
        }
    });
    let sum: u32 = values.map(|v| v as u32).sum();
    sum
}
#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_parse_file() {
        let rucksacks = parse_file(TEST_STR);
        assert_eq!(rucksacks.get(2).unwrap().0.len(), 9);
    }

    #[test]
    fn test_find_overlap() {
        let rucksacks = parse_file(TEST_STR);
        let overlaps: Vec<char> = find_overlaps(&rucksacks);
        assert_eq!(overlaps[0], 'p');
        assert_eq!(overlaps[1], 'L');
    }

    #[test]
    fn test_sum_overlaps() {
        let rucksacks = parse_file(TEST_STR);
        let overlaps: Vec<char> = find_overlaps(&rucksacks);
        assert_eq!(sum_overlaps(&overlaps), 157);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = super::read_file(module_path!());

    let rucksacks = parse_file(&contents);
    let overlaps: Vec<char> = find_overlaps(&rucksacks);
    println!("{}", sum_overlaps(&overlaps));
}

#[allow(dead_code)]
pub fn part2() {
    // let contents = super::read_file(module_path!());

    // println!("{}", score_rounds(parse_file_2(contents.as_str())))
}

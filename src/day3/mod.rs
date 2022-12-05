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

fn find_elf_overlap(rucksacks: (&Rucksack, &Rucksack, &Rucksack)) -> char {
    let pack1 = [rucksacks.0 .0.clone(), rucksacks.0 .1.clone()].concat();

    let pack2 = [rucksacks.1 .0.clone(), rucksacks.1 .1.clone()].concat();
    let pack3 = [rucksacks.2 .0.clone(), rucksacks.2 .1.clone()].concat();
    let intersect1 = pack1
        .iter()
        .collect::<HashSet<_>>()
        .intersection(&pack2.iter().collect::<HashSet<_>>())
        .map(|c| *c)
        .collect::<HashSet<_>>();

    let pack3_set = pack3.iter().collect::<HashSet<_>>();
    let intersect = intersect1.intersection(&pack3_set);

    ***intersect.collect::<Vec<_>>().first().unwrap()
}

fn find_elf_overlaps(rucksacks: &Vec<Rucksack>) -> Vec<char> {
    rucksacks
        .chunks(3)
        .map(|chunk| find_elf_overlap((&chunk[0], &chunk[1], &chunk[2])))
        .collect()
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

    #[test]
    fn test_find_elf_overlap() {
        let rucksacks = parse_file(TEST_STR);
        let overlap: char = find_elf_overlap((&rucksacks[0], &rucksacks[1], &rucksacks[2]));
        assert_eq!(overlap, 'r');
    }

    #[test]
    fn test_sum_elf_overlaps() {
        let rucksacks = parse_file(TEST_STR);
        let overlaps: Vec<char> = find_elf_overlaps(&rucksacks);
        assert_eq!(sum_overlaps(&overlaps), 70);
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
    let contents = super::read_file(module_path!());
    let rucksacks = parse_file(&contents);
    let overlaps: Vec<char> = find_elf_overlaps(&rucksacks);

    println!("{}", sum_overlaps(&overlaps));
}

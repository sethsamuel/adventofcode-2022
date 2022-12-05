use std::ops::RangeInclusive;

use crate::file::read_file;

trait ContainsRange<Idx> {
    fn contains_range(&self, range: &RangeInclusive<Idx>) -> bool;
}

impl<Idx> ContainsRange<Idx> for RangeInclusive<Idx>
where
    Idx: PartialOrd<Idx>,
{
    fn contains_range(&self, range: &RangeInclusive<Idx>) -> bool {
        self.contains(range.start()) && self.contains(range.end())
    }
}

trait OverlapsRange<Idx> {
    fn overlaps_range(&self, range: &RangeInclusive<Idx>) -> bool;
}

impl<Idx> OverlapsRange<Idx> for RangeInclusive<Idx>
where
    Idx: PartialOrd<Idx>,
{
    fn overlaps_range(&self, range: &RangeInclusive<Idx>) -> bool {
        range.contains_range(self) || self.contains(&range.start()) || self.contains(&range.end())
    }
}

type Elf = RangeInclusive<u32>;
type ElfPair = (Elf, Elf);

fn parse_file(text: &str) -> Vec<ElfPair> {
    let lines = text.lines();
    lines
        .map(|l| {
            let elfs: Vec<RangeInclusive<u32>> = l
                .split(",")
                .map(|e| {
                    let range: Vec<u32> = e.split("-").map(|i| i.parse::<u32>().unwrap()).collect();
                    range[0]..=range[1]
                })
                .collect();
            (elfs[0].clone(), elfs[1].clone())
        })
        .collect()
}

fn get_redundant_elfs(elfs: &Vec<ElfPair>) -> Vec<ElfPair> {
    elfs.iter()
        .filter(|pair| pair.0.contains_range(&pair.1) || pair.1.contains_range(&pair.0))
        .map(|p| p.clone())
        .collect()
}

fn get_overlap_elfs(elfs: &Vec<ElfPair>) -> Vec<ElfPair> {
    elfs.iter()
        .filter(|pair| pair.0.overlaps_range(&pair.1))
        .map(|p| p.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_contains_range() {
        assert!((0..=2).contains_range(&(0..=1)));
        assert!((0..=2).contains_range(&(0..=2)));
        assert!((0..=2).contains_range(&(1..=1)));
        assert!((0..=2).contains_range(&(1..=2)));
        assert!(!(0..=2).contains_range(&(0..=3)));
        assert!(!(0..=2).contains_range(&(3..=4)));
    }

    #[test]
    fn test_overlaps_range() {
        assert!((0..=2).overlaps_range(&(0..=1)));
        assert!((0..=2).overlaps_range(&(0..=2)));
        assert!((0..=2).overlaps_range(&(1..=1)));
        assert!((0..=2).overlaps_range(&(1..=2)));
        assert!((0..=2).overlaps_range(&(0..=3)));
        assert!(!(0..=2).overlaps_range(&(3..=4)));
        assert!((0..=1).overlaps_range(&(0..=3)));
        assert!((2..=4).overlaps_range(&(0..=2)));
        assert!((2..=4).overlaps_range(&(0..=3)));
        assert!((2..=4).overlaps_range(&(0..=5)));
    }

    #[test]
    fn test_parse_file() {
        let elfs = parse_file(TEST_STR);
        assert_eq!(*elfs.get(2).unwrap().0.start(), 5);
        assert_eq!(*elfs.get(4).unwrap().1.end(), 6);
    }

    #[test]
    fn test_get_redundant_elfs() {
        let elfs = parse_file(TEST_STR);
        let redundant_elfs = get_redundant_elfs(&elfs);
        assert_eq!(redundant_elfs.len(), 2);
    }

    #[test]
    fn test_get_overlap_elfs() {
        let elfs = parse_file(TEST_STR);
        let overlap_elfs = get_overlap_elfs(&elfs);
        assert_eq!(overlap_elfs.len(), 4);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let elfs = parse_file(&contents);
    let redundant_elfs = get_redundant_elfs(&elfs);

    println!("{}", redundant_elfs.len());
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let elfs = parse_file(&contents);
    let overlap_elfs = get_overlap_elfs(&elfs);

    println!("{}", overlap_elfs.len());
}

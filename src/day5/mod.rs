extern crate regex;
use std::fmt;

use regex::Regex;

use crate::file::read_file;

#[derive(Debug)]
struct Crate {
    id: char,
}

struct Stack {
    crates: Vec<Crate>,
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            self.crates.iter().map(|c| c.id).collect::<Vec<char>>()
        )
    }
}

struct Instruction {
    quantity: u32,
    from: u8,
    to: u8,
}
struct Input {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

fn parse_file(text: &str) -> Input {
    // let lines = text.lines();
    let text_parts: Vec<&str> = text.split("\n\n").collect();
    let stacks_str = text_parts.first().unwrap();
    let stacks_count = (stacks_str.lines().last().unwrap().len() + 1) / 4;
    let mut stacks: Vec<Stack> = (0..stacks_count)
        .map(|_| Stack { crates: Vec::new() })
        .collect();
    println!("Stacks count {}", stacks_count);
    stacks_str.lines().rev().skip(1).for_each(|l| {
        for i in 0..stacks_count {
            let crate_id = l
                .get(i * 4 + 1..i * 4 + 1 + 1)
                .unwrap()
                .chars()
                .last()
                .unwrap();
            if crate_id == ' ' {
                continue;
            }
            println!("crate id {}", crate_id);
            stacks[i].crates.push(Crate { id: crate_id });
            println!("crated {:?}", stacks[i]);
        }
    });

    let instructions_str = text_parts.last().unwrap();
    let instructions_regex = Regex::new(r"move (?P<q>\d+) from (?P<f>\d+) to (?P<t>\d+)").unwrap();
    let instructions: Vec<Instruction> = instructions_str
        .lines()
        .map(|l| {
            let captures = instructions_regex.captures(l).unwrap();
            Instruction {
                quantity: captures.name("q").unwrap().as_str().parse::<u32>().unwrap(),
                from: captures.name("f").unwrap().as_str().parse::<u8>().unwrap(),
                to: captures.name("t").unwrap().as_str().parse::<u8>().unwrap(),
            }
        })
        .collect();

    Input {
        stacks,
        instructions,
    }
}

fn execute_instructions(input: &mut Input) {
    input.instructions.iter().for_each(|i| {
        for _ in 1..=i.quantity {
            let c = input
                .stacks
                .get_mut(i.from as usize - 1)
                .unwrap()
                .crates
                .pop()
                .unwrap();
            input
                .stacks
                .get_mut(i.to as usize - 1)
                .unwrap()
                .crates
                .push(c);
            print!("{:?}\n\n", input.stacks);
        }
    });
}

fn get_top_crates(input: Input) -> Vec<char> {
    input
        .stacks
        .iter()
        .map(|s| s.crates.last().unwrap().id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse_file() {
        let input = parse_file(TEST_STR);
        assert_eq!(input.instructions.len(), 4);
        assert_eq!(input.instructions.first().unwrap().from, 2);
        assert_eq!(input.instructions.last().unwrap().quantity, 1);
        assert_eq!(input.stacks.len(), 3);
        assert_eq!(input.stacks.first().unwrap().crates.len(), 2);
        assert_eq!(
            input.stacks.first().unwrap().crates.first().unwrap().id,
            'Z'
        );
    }

    #[test]
    fn test_execute_instructions() {
        let mut input = parse_file(TEST_STR);
        println!("{:?}", input.stacks);
        execute_instructions(&mut input);
        assert_eq!(input.stacks.first().unwrap().crates.len(), 1);
        assert_eq!(input.stacks.get(1).unwrap().crates.len(), 1);
        assert_eq!(input.stacks.get(2).unwrap().crates.len(), 4);
        assert_eq!(input.stacks.get(2).unwrap().crates.last().unwrap().id, 'Z');
    }

    #[test]
    fn test_get_top_crates() {
        let mut input = parse_file(TEST_STR);
        execute_instructions(&mut input);
        assert_eq!(get_top_crates(input), ['C', 'M', 'Z']);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let mut input = parse_file(&contents);
    execute_instructions(&mut input);
    println!("{:?}", String::from_iter(get_top_crates(input).iter()));
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    // println!("{}", overlap_elfs.len());
}

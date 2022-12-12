use std::ops::{AddAssign, DivAssign, MulAssign, Rem};

use crate::file::read_file;

#[derive(Debug, Default, Clone, PartialEq)]
enum Operand {
    #[default]
    Mult,
    Add,
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        match s {
            "*" => Self::Mult,
            "+" => Self::Add,
            _ => panic!("Unknown operand {}", s),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
enum Argument {
    Constant(usize),
    #[default]
    Old,
}

impl From<&str> for Argument {
    fn from(s: &str) -> Self {
        match s {
            "old" => Self::Old,
            i => Self::Constant(i.parse::<usize>().unwrap()),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Operation {
    operand: Operand,
    // lhs: Argument,
    rhs: Argument,
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: Vec<usize>,
    divisor: usize,
    true_target: usize,
    false_target: usize,
    operation: Operation,
    inspections: usize,
}

fn parse_file(text: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let lines = text.lines();
    let mut current_monkey = Monkey::default();
    for line in lines {
        if line.is_empty() {
            monkeys.push(current_monkey);
            current_monkey = Monkey::default();
        } else if line.starts_with("Monkey") {
            //Noop
        } else {
            let mut parts = line.split(":").map(|s| s.trim());
            match parts.next() {
                Some("Starting items") => {
                    current_monkey.items = parts
                        .next()
                        .unwrap()
                        .split(",")
                        .map(|i| i.trim().parse::<usize>().unwrap())
                        .collect()
                }
                Some("Operation") => {
                    let operation_parts: Vec<&str> = parts.next().unwrap().split(" ").collect();
                    current_monkey.operation = Operation {
                        operand: operation_parts[3].into(),
                        // lhs: operation_parts[2].into(),
                        rhs: operation_parts[4].into(),
                    }
                }
                Some("Test") => {
                    current_monkey.divisor = parts
                        .next()
                        .unwrap()
                        .split(" ")
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                }
                Some("If true") => {
                    current_monkey.true_target = parts
                        .next()
                        .unwrap()
                        .split(" ")
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                }
                Some("If false") => {
                    current_monkey.false_target = parts
                        .next()
                        .unwrap()
                        .split(" ")
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                }
                Some(s) => panic!("Unknown line {}", s),
                None => (),
            }
        }
    }
    monkeys.push(current_monkey);

    monkeys
}

fn get_monkey_business(monkeys: &mut Vec<Monkey>) -> usize {
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];
            let mut items = Vec::new();

            while monkey.items.len() > 0 {
                let mut item = monkey.items.remove(0);
                match monkey.operation.operand {
                    Operand::Mult => item.mul_assign(match monkey.operation.rhs {
                        Argument::Constant(c) => c,
                        Argument::Old => item.clone(),
                    }),
                    Operand::Add => item.add_assign(match monkey.operation.rhs {
                        Argument::Constant(c) => c,
                        Argument::Old => item.clone(),
                    }),
                }
                item.div_assign(3);

                if item.rem(monkey.divisor) == 0 {
                    items.push((monkey.true_target, item));
                } else {
                    items.push((monkey.false_target, item));
                }
            }
            monkey.items.truncate(0);
            monkey.inspections += items.len();
            for item in items {
                monkeys[item.0].items.push(item.1);
            }
        }
    }
    monkeys.sort_by(|a, b| a.inspections.partial_cmp(&b.inspections).unwrap().reverse());
    monkeys[0].inspections * monkeys[1].inspections
}

fn get_more_monkey_business(monkeys: &mut Vec<Monkey>) -> usize {
    let gcd: usize = monkeys.iter().map(|m| m.divisor).product();
    for _ in 0..10_000 {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];
            let mut items = Vec::new();

            while monkey.items.len() > 0 {
                let mut item = monkey.items.remove(0);
                match monkey.operation.operand {
                    Operand::Mult => item.mul_assign(match monkey.operation.rhs {
                        Argument::Constant(c) => c,
                        Argument::Old => item.clone(),
                    }),
                    Operand::Add => item.add_assign(match monkey.operation.rhs {
                        Argument::Constant(c) => c,
                        Argument::Old => item.clone(),
                    }),
                }

                if item.rem(monkey.divisor) == 0 {
                    items.push((monkey.true_target, item));
                } else {
                    items.push((monkey.false_target, item));
                }
            }
            monkey.items.truncate(0);
            monkey.inspections += items.len();
            for item in items {
                monkeys[item.0].items.push(item.1 % gcd);
            }
        }
    }
    monkeys.sort_by(|a, b| a.inspections.partial_cmp(&b.inspections).unwrap().reverse());
    monkeys[0].inspections * monkeys[1].inspections
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parse_file() {
        let monkeys = parse_file(TEST_STR);
        assert_eq!(monkeys.len(), 4);
        assert_eq!(monkeys[0].items.len(), 2);
        assert_eq!(monkeys[2].true_target, 1);
        assert_eq!(monkeys[3].operation.operand, Operand::Add);
        // assert_eq!(monkeys[3].operation.lhs, Argument::Old);
        assert_eq!(monkeys[3].operation.rhs, Argument::Constant(3));
    }

    #[test]
    fn test_get_monkey_business() {
        let mut monkeys = parse_file(TEST_STR);
        let mb = get_monkey_business(&mut monkeys);
        assert_eq!(mb, 10605);
    }

    #[test]
    fn test_get_more_monkey_business() {
        let mut monkeys = parse_file(TEST_STR);
        let mb = get_more_monkey_business(&mut monkeys);
        assert_eq!(mb, 2713310158);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let mut monkeys = parse_file(&contents);
    let mb = get_monkey_business(&mut monkeys);
    println!("{}", mb);
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let mut monkeys = parse_file(&contents);
    let mb = get_more_monkey_business(&mut monkeys);
    println!("{}", mb);
}

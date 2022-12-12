use std::collections::HashSet;

use crate::file::read_file;

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "L" => Self::Left,
            "R" => Self::Right,
            "D" => Self::Down,
            _ => panic!("Unknown direction {}", s),
        }
    }
}
#[derive(Debug)]
struct Instruction {
    pub direction: Direction,
    pub distance: usize,
}

#[derive(Hash, Debug, Default, PartialEq, Eq, Clone)]
struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    fn move_by(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn move_towards(&mut self, position: &Position) {
        let vector = (position.x - self.x, position.y - self.y);
        if vector.0.abs() == 2 && vector.1 == 0 {
            self.x += vector.0.clamp(-1, 1);
        } else if vector.1.abs() == 2 && vector.0 == 0 {
            self.y += vector.1.clamp(-1, 1);
        } else if vector.0.abs() + vector.1.abs() >= 3 {
            self.x += vector.0.clamp(-1, 1);
            self.y += vector.1.clamp(-1, 1);
        }
    }
}

fn parse_file(text: &str) -> Vec<Instruction> {
    let instructions: Vec<Instruction> = text
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            Instruction {
                direction: parts[0].into(),
                distance: parts[1].parse::<usize>().unwrap(),
            }
        })
        .collect();

    instructions
}

fn count_tail_positions(instructions: &Vec<Instruction>) -> usize {
    let mut head = Position {
        ..Default::default()
    };
    let mut tail = Position {
        ..Default::default()
    };
    let mut positions: HashSet<Position> = HashSet::new();
    positions.insert(tail.clone());

    for inst in instructions.iter() {
        // println!("{:?}", inst);
        for _ in 0..inst.distance {
            head.move_by(&inst.direction);
            // println!("H {:?}", head);
            tail.move_towards(&head);
            // println!("T {:?}", tail);
            positions.insert(tail.clone());
        }
    }

    positions.len()
}

fn count_long_tail_positions(instructions: &Vec<Instruction>) -> usize {
    let mut rope: Vec<Position> = (0..10)
        .map(|_| Position {
            ..Default::default()
        })
        .collect();
    let mut positions: HashSet<Position> = HashSet::new();
    positions.insert(rope[9].clone());

    for inst in instructions.iter() {
        // println!("{:?}", inst);
        for _ in 0..inst.distance {
            rope[0].move_by(&inst.direction);
            for k in 1..rope.len() {
                let next_knot = rope[k - 1].clone();
                rope[k].move_towards(&next_knot);
            }
            // println!("{:?}", rope);
            positions.insert(rope[9].clone());
        }
    }

    positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static TEST_STR_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_parse_file() {
        let instructions = parse_file(TEST_STR);
        assert_eq!(instructions.len(), 8);
        assert_eq!(instructions[3].distance, 1);
        assert_eq!(instructions[6].direction, Direction::Left);
    }

    #[test]
    fn test_count_tail_positions() {
        let instructions = parse_file(TEST_STR);
        assert_eq!(count_tail_positions(&instructions), 13);
    }

    #[test]
    fn test_count_long_tail_positions() {
        let instructions = parse_file(TEST_STR_2);
        assert_eq!(count_long_tail_positions(&instructions), 36);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let instructions = parse_file(&contents);

    println!("{:?}", count_tail_positions(&instructions));
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let instructions = parse_file(&contents);

    println!("{:?}", count_long_tail_positions(&instructions));
}

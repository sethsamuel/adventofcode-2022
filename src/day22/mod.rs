use std::collections::HashMap;

use crate::file::read_file;

#[derive(Debug, Clone, PartialEq)]
enum GridState {
    Void,
    Path,
    Wall,
}
#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Move(usize),
    Right,
    Left,
}

type Grid = Vec<Vec<GridState>>;

fn parse_file(text: &str) -> (Grid, Vec<Instruction>) {
    let mut lines: Vec<&str> = text.lines().collect();
    let instruction_line = lines.pop().unwrap();
    lines.pop();
    let mut instructions = Vec::new();
    let mut char_buffer: Vec<char> = Vec::new();
    for c in instruction_line.chars() {
        if c == 'L' || c == 'R' {
            instructions.push(Instruction::Move(
                char_buffer
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            ));
            char_buffer.truncate(0);
        }
        match c {
            'L' => instructions.push(Instruction::Left),
            'R' => instructions.push(Instruction::Right),
            _ => char_buffer.push(c),
        }
    }
    instructions.push(Instruction::Move(
        char_buffer
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap(),
    ));

    let mut grid: Grid = Vec::new();
    lines.iter().for_each(|l| {
        let chars = l.chars();
        let mut row = Vec::new();
        for c in chars {
            row.push(match c {
                ' ' => GridState::Void,
                '.' => GridState::Path,
                '#' => GridState::Wall,
                _ => panic!("Unknown char {}", c),
            });
        }

        grid.push(row);
    });

    (grid, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_parse_file() {
        let (grid, instructions) = parse_file(TEST_STR);
        assert_eq!(grid[0][0], GridState::Void);
        assert_eq!(grid[1][8], GridState::Path);
        assert_eq!(grid[1][9], GridState::Wall);
        assert_eq!(instructions[0], Instruction::Move(10));
        assert_eq!(instructions[1], Instruction::Right);
        assert_eq!(instructions[2], Instruction::Move(5));
        assert_eq!(*instructions.last().unwrap(), Instruction::Move(5));
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    // let (mut grid, sensors) = parse_file(&contents);
    // println!("{}", count_eliminated(&mut grid, &sensors, 2_000_000));
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    // let (mut grid, sensors) = parse_file(&contents);
    // println!(
    //     "{}",
    //     find_tuning(&mut grid, &sensors, 4_000_000, 4_000_000).unwrap()
    // );
}

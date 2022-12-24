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

#[derive(Debug, Clone, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn value(&self) -> usize {
        match self {
            Facing::North => 3,
            Facing::South => 1,
            Facing::East => 0,
            Facing::West => 2,
        }
    }
    fn turn(&self, direction: &Instruction) -> Facing {
        match direction {
            Instruction::Move(_) => self.clone(),
            Instruction::Right => match self {
                Facing::North => Facing::East,
                Facing::South => Facing::West,
                Facing::East => Facing::South,
                Facing::West => Facing::North,
            },
            Instruction::Left => match self {
                Facing::North => Facing::West,
                Facing::South => Facing::East,
                Facing::East => Facing::North,
                Facing::West => Facing::South,
            },
        }
    }
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

fn get_next_position_map(
    grid: &Grid,
    position: &(usize, usize),
    facing: &Facing,
) -> (usize, usize) {
    match facing {
        Facing::North => {
            if position.0 == 0
                || position.1 > grid[position.0 - 1].len() - 1
                || grid[position.0 - 1][position.1] == GridState::Void
            {
                let valid_position = grid
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_, col)| {
                        position.1 < col.len() - 1 && col[position.1] != GridState::Void
                    })
                    .unwrap();
                (grid.len() - 1 - valid_position.0, position.1)
            } else {
                (position.0 - 1, position.1)
            }
        }
        Facing::South => {
            if position.0 == grid.len() - 1
                || position.1 > grid[position.0 + 1].len() - 1
                || grid[position.0 + 1][position.1] == GridState::Void
            {
                let valid_position = grid
                    .iter()
                    .enumerate()
                    .find(|(_, col)| {
                        position.1 < col.len() - 1 && col[position.1] != GridState::Void
                    })
                    .unwrap();
                (valid_position.0, position.1)
            } else {
                (position.0 + 1, position.1)
            }
        }
        Facing::East => {
            if position.1 == grid[position.0].len() - 1
                || grid[position.0][position.1 + 1] == GridState::Void
            {
                let valid_position = grid[position.0]
                    .iter()
                    .enumerate()
                    .find(|(i, col)| **col != GridState::Void)
                    .unwrap();
                (position.0, valid_position.0)
            } else {
                (position.0, position.1 + 1)
            }
        }
        Facing::West => {
            if position.1 == 0 || grid[position.0][position.1 - 1] == GridState::Void {
                let valid_position = grid[position.0]
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(i, col)| **col != GridState::Void)
                    .unwrap();
                (position.0, grid[position.0].len() - 1 - valid_position.0)
            } else {
                (position.0, position.1 - 1)
            }
        }
    }
}

fn get_next_position_cube(
    grid: &Grid,
    position: &(usize, usize),
    facing: &Facing,
) -> ((usize, usize), Facing) {
    let cube_size = grid
        .iter()
        .enumerate()
        .find(|(_, col)| col[0] != GridState::Void)
        .unwrap()
        .0;

    match facing {
        Facing::North => {
            if position.0 == 0 {
                //1 -> 5
                ((grid.len() - 1, position.1), *facing)
            } else if position.1 > grid[position.0 - 1].len() - 1 {
                //6 -> 4
                (
                    (
                        cube_size * 2 - (position.1 - cube_size * 3),
                        cube_size * 3 - 1,
                    ),
                    Facing::West,
                )
            } else if grid[position.0 - 1][position.1] == GridState::Void {
                if position.1 < cube_size {
                    //3 -> 1
                    todo!()
                } else {
                    //2 -> 1
                    todo!()
                }
            } else {
                ((position.0 - 1, position.1), facing)
            }
        }
        Facing::South => {
            if position.0 == grid.len() - 1
                || position.1 > grid[position.0 + 1].len() - 1
                || grid[position.0 + 1][position.1] == GridState::Void
            {
                let valid_position = grid
                    .iter()
                    .enumerate()
                    .find(|(_, col)| {
                        position.1 < col.len() - 1 && col[position.1] != GridState::Void
                    })
                    .unwrap();
                (valid_position.0, position.1)
            } else {
                (position.0 + 1, position.1)
            }
        }
        Facing::East => {
            if position.1 == grid[position.0].len() - 1
                || grid[position.0][position.1 + 1] == GridState::Void
            {
                let valid_position = grid[position.0]
                    .iter()
                    .enumerate()
                    .find(|(i, col)| **col != GridState::Void)
                    .unwrap();
                (position.0, valid_position.0)
            } else {
                (position.0, position.1 + 1)
            }
        }
        Facing::West => {
            if position.1 == 0 || grid[position.0][position.1 - 1] == GridState::Void {
                let valid_position = grid[position.0]
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(i, col)| **col != GridState::Void)
                    .unwrap();
                (position.0, grid[position.0].len() - 1 - valid_position.0)
            } else {
                (position.0, position.1 - 1)
            }
        }
    }
}

fn get_next_position(
    is_cube: bool,
    grid: &Grid,
    position: &(usize, usize),
    facing: &Facing,
) -> (usize, usize) {
    if is_cube {
        get_next_position_cube(grid, position, facing)
    } else {
        get_next_position_map(grid, position, facing)
    }
}

fn execute(
    is_cube: bool,
    grid: &Grid,
    instructions: &Vec<Instruction>,
) -> ((usize, usize), Facing) {
    let mut facing = Facing::East;
    let mut position = (0, 0);
    for instruction in instructions.iter() {
        match instruction {
            Instruction::Move(d) => {
                for _ in 0..*d {
                    let next_position = get_next_position(is_cube, grid, &position, &facing);
                    match grid[next_position.0][next_position.1] {
                        GridState::Void => panic!("Ran into void!"),
                        GridState::Path => position = next_position,
                        GridState::Wall => break,
                    }
                }
            }
            _ => facing = facing.turn(instruction),
        }
    }
    (position, facing)
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

    #[test]
    fn test_get_next_position_map() {
        let (grid, _) = parse_file(TEST_STR);
        assert_eq!(get_next_position_map(&grid, &(0, 8), &Facing::East), (0, 9));
        assert_eq!(
            get_next_position_map(&grid, &(0, 8), &Facing::West),
            (0, 11)
        );
        assert_eq!(
            get_next_position_map(&grid, &(0, 8), &Facing::South),
            (1, 8)
        );
        assert_eq!(
            get_next_position_map(&grid, &(0, 8), &Facing::North),
            (11, 8)
        );
    }

    #[test]
    fn test_execute() {
        let (grid, instructions) = parse_file(TEST_STR);
        let (position, facing) = execute(false, &grid, &instructions);
        assert_eq!(position.0, 5);
        assert_eq!(position.1, 7);
        assert_eq!(facing, Facing::East);
        assert_eq!(
            (position.0 + 1) * 1000 + (position.1 + 1) * 4 + facing.value(),
            6032
        )
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let (grid, instructions) = parse_file(&contents);
    let (position, facing) = execute(&grid, &instructions);

    println!(
        "{}",
        (position.0 + 1) * 1000 + (position.1 + 1) * 4 + facing.value()
    );
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

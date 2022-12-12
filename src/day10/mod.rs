use crate::file::read_file;

#[derive(Debug, PartialEq)]
enum Command {
    Add,
    Noop,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "noop" => Self::Noop,
            "addx" => Self::Add,
            _ => panic!("Unknown command {}", s),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    pub command: Command,
    pub argument: Option<i32>,
}

fn parse_file(text: &str) -> Vec<Instruction> {
    let instructions: Vec<Instruction> = text
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            Instruction {
                command: parts[0].into(),
                argument: parts.get(1).unwrap_or(&"").parse::<i32>().ok(),
            }
        })
        .collect();

    instructions
}

fn execute(instructions: &Vec<Instruction>) -> Vec<i32> {
    let mut cycle = 0;
    let mut register: i32 = 1;

    let mut signals: Vec<i32> = Vec::new();

    for instr in instructions.iter() {
        match instr.command {
            Command::Add => {
                cycle += 1;
                if cycle == 20 || (cycle - 20) % 40 == 0 {
                    signals.push(register * cycle);
                }
                cycle += 1;
                if cycle == 20 || (cycle - 20) % 40 == 0 {
                    signals.push(register * cycle);
                }
                register += instr.argument.unwrap_or(0);
            }
            Command::Noop => {
                cycle += 1;
                if cycle == 20 || (cycle - 20) % 40 == 0 {
                    signals.push(register * cycle);
                }
            }
        }
    }

    signals
}

type Screen = Vec<Vec<bool>>;

fn set_pixel(cycle: usize, register: i32, screen: &mut Screen) {
    let line = (cycle - 1) / screen[0].len();
    let pixel = (cycle - 1) % screen[0].len();

    screen[line][pixel] = (register - pixel as i32).abs() <= 1;
}

fn draw(instructions: &Vec<Instruction>) -> Screen {
    let mut cycle: usize = 0;
    let mut register: i32 = 1;

    let mut screen: Screen = (0..6).map(|_| (0..40).map(|_| false).collect()).collect();

    for instr in instructions.iter() {
        match instr.command {
            Command::Add => {
                cycle += 1;
                set_pixel(cycle, register, &mut screen);
                cycle += 1;
                set_pixel(cycle, register, &mut screen);
                register += instr.argument.unwrap_or(0);
            }
            Command::Noop => {
                cycle += 1;
                set_pixel(cycle, register, &mut screen);
            }
        }
    }

    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_parse_file() {
        let instructions = parse_file(TEST_STR);
        assert_eq!(instructions.len(), 146);
        assert_eq!(instructions[2].command, Command::Add);
    }

    #[test]
    fn test_execute() {
        let instructions = parse_file(TEST_STR);
        let signals = execute(&instructions);
        assert_eq!(signals.len(), 6);
        assert_eq!(signals[0], 420);
        assert_eq!(signals[5], 3960);
    }

    #[test]
    fn test_draw() {
        let instructions = parse_file(TEST_STR);
        let screen = draw(&instructions);
        for line in screen {
            println!(
                "{}",
                line.iter()
                    .map(|p| if *p { '#' } else { '.' })
                    .collect::<String>()
            )
        }
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let instructions = parse_file(&contents);
    let signals = execute(&instructions);

    println!("{:?}", signals.iter().sum::<i32>());
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let instructions = parse_file(&contents);

    let screen = draw(&instructions);
    for line in screen {
        println!(
            "{}",
            line.iter()
                .map(|p| if *p { '#' } else { '.' })
                .collect::<String>()
        )
    }
}

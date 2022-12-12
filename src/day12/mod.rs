use crate::file::read_file;

fn parse_file(text: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let grid = text
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (y, x);
                        return 0;
                    }
                    if c == 'E' {
                        end = (y, x);
                        return 25;
                    }
                    c as u8 - 96
                })
                .collect()
        })
        .collect();

    (grid, start, end)
}

fn get_shortest_walk(grid: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_parse_file() {
        let (grid, start, end) = parse_file(TEST_STR);
        assert_eq!(start, (0, 0));
        assert_eq!(end, (2, 5));
        assert_eq!(grid[0][2], 2);
        assert_eq!(grid[3][2], 3);
    }

    #[test]
    fn test_get_shortest_walk() {
        let (grid, start, end) = parse_file(TEST_STR);
        assert_eq!(get_shortest_walk(&grid, start, end), 31);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    // let mut monkeys = parse_file(&contents);
    // let mb = get_monkey_business(&mut monkeys);
    // println!("{}", mb);
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    // let mut monkeys = parse_file(&contents);
    // let mb = get_more_monkey_business(&mut monkeys);
    // println!("{}", mb);
}

use std::vec;

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
                        return 1;
                    }
                    if c == 'E' {
                        end = (y, x);
                        return 26;
                    }
                    c as u8 - 96
                })
                .collect()
        })
        .collect();

    (grid, start, end)
}

fn get_shortest_walk(grid: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut distances = (0..grid.len())
        .map(|i| {
            (0..grid[i].len())
                .map(|_| usize::MAX)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    distances[start.0][start.1] = 0;

    let mut queue: Vec<(usize, usize)> = vec![start];
    while let Some(current) = queue.pop() {
        let current_value = grid[current.0][current.1];
        let current_distance = distances[current.0][current.1];

        for vector in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let coords: (usize, usize) = (
                (current.0 as isize + vector.0) as usize,
                (current.1 as isize + vector.1) as usize,
            );
            if (0..grid.len()).contains(&(coords.0))
                && (0..grid[current.0].len()).contains(&(coords.1))
            {
                let value = grid[coords.0][coords.1];
                let distance = distances[coords.0][coords.1];
                if value <= current_value + 1 && (distance > current_distance + 1) {
                    distances[coords.0][coords.1] = current_distance + 1;
                    queue.push((coords.0, coords.1));
                }
            }
        }
    }

    // for d in distances.clone() {
    //     println!(
    //         "{}",
    //         d.iter()
    //             .map(|d| match *d {
    //                 usize::MAX => "X".to_string(),
    //                 _ => format!(" {:0>2} ", d.to_string()),
    //             })
    //             .collect::<String>()
    //     );
    // }

    distances[end.0][end.1]
}

fn get_shortest_trailhead(grid: &Vec<Vec<u8>>, end: (usize, usize)) -> usize {
    let mut shortest_distance = usize::MAX;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 1 {
                let distance = get_shortest_walk(grid, (row, col), end);
                // println!("{:?} {}", (row, col), distance);
                if distance < shortest_distance {
                    // println!("{:?}", (row, col));
                    shortest_distance = distance;
                }
            }
        }
    }
    shortest_distance
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
        assert_eq!(grid[2][5], 26);
    }

    #[test]
    fn test_get_shortest_walk() {
        let (grid, start, end) = parse_file(TEST_STR);
        assert_eq!(get_shortest_walk(&grid, start, end), 31);
    }

    #[test]
    fn test_get_shortest_trailhead() {
        let (grid, start, end) = parse_file(TEST_STR);

        assert_eq!(get_shortest_trailhead(&grid, end), 29);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let (grid, start, end) = parse_file(&contents);
    let walk = get_shortest_walk(&grid, start, end);

    println!("{}", walk);
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let (grid, _start, end) = parse_file(&contents);
    let start = get_shortest_trailhead(&grid, end);

    println!("{}", start);
}

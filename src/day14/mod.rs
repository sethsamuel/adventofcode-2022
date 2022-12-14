use crate::file::read_file;

#[derive(Debug, Clone, PartialEq)]
enum PathState {
    Air,
    Sand,
    Rock,
}

type Grid = Vec<Vec<PathState>>;

fn parse_file(text: &str) -> Grid {
    let mut grid: Grid = (0..1000)
        .map(|_| (0..1000).map(|_| PathState::Air).collect())
        .collect();

    text.lines().for_each(|l| {
        let mut coords = l.split(" -> ");
        let mut first_coord = coords
            .next()
            .unwrap()
            .split(",")
            .map(|c| c.parse::<usize>().unwrap());

        let mut current_x = first_coord.next().unwrap();
        let mut current_y = first_coord.next().unwrap();
        for coord in coords {
            let mut parts = coord.split(",").map(|c| c.parse::<usize>().unwrap());
            let target_x = parts.next().unwrap();
            let target_y = parts.next().unwrap();
            // println!("{current_x},{current_y}->{target_x},{target_y}");

            for x in current_x.min(target_x)..=target_x.max(current_x) {
                for y in current_y.min(target_y)..=target_y.max(current_y) {
                    // println!("{x},{y}");
                    grid[x][y] = PathState::Rock;
                }
            }

            current_x = target_x;
            current_y = target_y;
        }
    });

    grid
}

fn add_floor(grid: &mut Grid) -> &Grid {
    let mut max_y = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == PathState::Rock && y > max_y {
                max_y = y;
            }
        }
    }

    for x in 0..grid.len() {
        grid[x][max_y + 2] = PathState::Rock
    }

    grid
}

fn move_sand(grid: &Grid, sand: (usize, usize)) -> Option<(usize, usize)> {
    if sand.1 >= grid[sand.0].len() - 1 {
        return None;
    }
    if grid[sand.0][sand.1 + 1] == PathState::Air {
        return Some((sand.0, sand.1 + 1));
    }
    if grid[sand.0 - 1][sand.1 + 1] == PathState::Air {
        return Some((sand.0 - 1, sand.1 + 1));
    }
    if grid[sand.0 + 1][sand.1 + 1] == PathState::Air {
        return Some((sand.0 + 1, sand.1 + 1));
    }

    return Some(sand);
}

fn drop_sand(grid: &mut Grid) -> usize {
    let mut sand_count = 0;
    let mut current_sand = (500, 0);
    loop {
        let new_sand = move_sand(grid, current_sand);
        match new_sand {
            Some(sand) => {
                if sand == current_sand {
                    grid[sand.0][sand.1] = PathState::Sand;
                    sand_count += 1;
                    if current_sand == (500, 0) {
                        return sand_count;
                    }
                    current_sand = (500, 0);
                } else {
                    current_sand = sand;
                }
            }
            None => return sand_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_parse_file() {
        let grid = parse_file(TEST_STR);
        assert_eq!(grid[498][4], PathState::Rock);
        assert_eq!(grid[498][5], PathState::Rock);
        assert_eq!(grid[499][6], PathState::Air);
        assert_eq!(grid[494][9], PathState::Rock);
    }

    #[test]
    fn test_drop_sand() {
        let mut grid = parse_file(TEST_STR);
        assert_eq!(drop_sand(&mut grid), 24);
    }

    #[test]
    fn test_drop_sand_to_floor() {
        let mut grid = parse_file(TEST_STR);
        add_floor(&mut grid);
        assert_eq!(drop_sand(&mut grid), 93);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let mut grid = parse_file(&contents);
    println!("{}", drop_sand(&mut grid));
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let mut grid = parse_file(&contents);
    add_floor(&mut grid);
    println!("{}", drop_sand(&mut grid));
}

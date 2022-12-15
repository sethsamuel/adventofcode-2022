use std::collections::HashMap;

use regex::Regex;

use crate::file::read_file;

#[derive(Debug, Clone, PartialEq)]
enum GridState {
    Sensor,
    Beacon,
}

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    range: usize,
}

impl Sensor {
    fn distance(&self, x: isize, y: isize) -> usize {
        self.x.abs_diff(x) + self.y.abs_diff(y)
    }

    fn is_in_range(&self, x: isize, y: isize) -> bool {
        let distance = self.distance(x, y);
        distance <= self.range
    }
}

type Row = HashMap<isize, GridState>;
type Grid = HashMap<isize, Row>;

fn grid_set(grid: &mut Grid, x: isize, y: isize, v: GridState) {
    grid.entry(x).or_insert_with(|| HashMap::new()).insert(y, v);
}

fn parse_file(text: &str) -> (Grid, Vec<Sensor>) {
    let mut grid: Grid = HashMap::new();

    let regex = Regex::new(
        r"Sensor at x=(?P<sx>(-|\d)+), y=(?P<sy>(-|\d)+): closest beacon is at x=(?P<bx>(-|\d)+), y=(?P<by>(-|\d)+)",
    ).unwrap();
    let mut sensors = Vec::new();
    text.lines().for_each(|l| {
        let captures = regex.captures(l).unwrap();
        let sensor = (
            captures
                .name("sx")
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
            captures
                .name("sy")
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
        );

        let beacon = (
            captures
                .name("bx")
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
            captures
                .name("by")
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
        );
        grid_set(&mut grid, sensor.0, sensor.1, GridState::Sensor);
        grid_set(&mut grid, beacon.0, beacon.1, GridState::Beacon);
        sensors.push(Sensor {
            x: sensor.0,
            y: sensor.1,
            range: beacon.0.abs_diff(sensor.0) + beacon.1.abs_diff(sensor.1),
        })
    });

    (grid, sensors)
}

fn count_eliminated(grid: &mut Grid, sensors: &Vec<Sensor>, y: isize) -> usize {
    let mut count = 0;
    for x in -1_000_000..1_000_000 {
        if grid.get(&x).and_then(|c| c.get(&y)) == None {
            match sensors.iter().find(|s| s.is_in_range(x, y)) {
                Some(_) => count += 1,
                None => (),
            }
        }
    }

    count
}

fn find_beacon(
    grid: &Grid,
    sensors: &Vec<Sensor>,
    max_x: isize,
    max_y: isize,
) -> Option<(isize, isize)> {
    for sensor in sensors {
        for x in (-1 * sensor.range as isize - 1)..=(sensor.range as isize + 1) {
            for y in [-1, 1] {
                let x_y = (
                    sensor.x + x,
                    sensor.y + y * ((sensor.range as isize + 1).abs_diff(x) as isize),
                );
                if (0..=max_x).contains(&x_y.0)
                    && (0..=max_y).contains(&x_y.1)
                    && grid.get(&x_y.0).and_then(|c| c.get(&x_y.1)) == None
                {
                    match sensors.iter().find(|s| s.is_in_range(x_y.0, x_y.1)) {
                        Some(_) => (),
                        None => return Some(x_y),
                    }
                }
            }
        }
    }
    None
}
fn find_tuning(grid: &Grid, sensors: &Vec<Sensor>, max_x: isize, max_y: isize) -> Option<isize> {
    if let Some(beacon) = find_beacon(grid, sensors, max_x, max_y) {
        return Some(beacon.0 * 4000000 + beacon.1);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_parse_file() {
        let (grid, sensors) = parse_file(TEST_STR);
        assert_eq!(grid[&-2][&15], GridState::Beacon);
        assert_eq!(grid[&13][&2], GridState::Sensor);
        assert_eq!(grid[&21][&22], GridState::Beacon);
        assert_eq!(sensors[3].x, 12);
        assert_eq!(sensors[3].y, 14);
    }

    #[test]
    fn test_count_eliminated() {
        let (mut grid, sensors) = parse_file(TEST_STR);
        assert_eq!(count_eliminated(&mut grid, &sensors, 10), 26);
    }

    #[test]
    fn test_find_tuning() {
        let (grid, sensors) = parse_file(TEST_STR);
        assert_eq!(find_tuning(&grid, &sensors, 20, 20), Some(56000011));
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let (mut grid, sensors) = parse_file(&contents);
    println!("{}", count_eliminated(&mut grid, &sensors, 2_000_000));
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let (mut grid, sensors) = parse_file(&contents);
    println!(
        "{}",
        find_tuning(&mut grid, &sensors, 4_000_000, 4_000_000).unwrap()
    );
}

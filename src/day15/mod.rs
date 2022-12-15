use std::collections::HashMap;

use regex::Regex;

use crate::file::read_file;

#[derive(Debug, Clone, PartialEq)]
enum GridState {
    Sensor,
    Beacon,
    NoBeacon,
}

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    beacon_x: isize,
    beacon_y: isize,
    range: usize,
}

impl Sensor {
    fn is_in_range(&self, x: isize, y: isize) -> bool {
        let distance = self.x.abs_diff(x) + self.y.abs_diff(y);
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
            beacon_x: beacon.0,
            beacon_y: beacon.1,
            range: beacon.0.abs_diff(sensor.0) + beacon.1.abs_diff(sensor.1),
        })
    });

    (grid, sensors)
}

fn eliminate_squares(grid: &mut Grid, sensors: &Vec<Sensor>) {
    for sensor in sensors {
        let distance =
            (sensor.x.abs_diff(sensor.beacon_x) + sensor.y.abs_diff(sensor.beacon_y)) as isize;
        println!("Filling sensor {:?} {}", sensor, distance);
        for x in (sensor.x - distance)..=(sensor.x + distance) {
            // println!("x {}", x);
            for y in (sensor.y - (distance - sensor.x.abs_diff(x) as isize))
                ..=(sensor.y + (distance - sensor.x.abs_diff(x) as isize))
            {
                match grid.get(&x).and_then(|c| c.get(&y)) {
                    None => grid_set(grid, x, y, GridState::NoBeacon),
                    _ => (),
                }
            }
        }
    }
}

fn count_eliminated(grid: &mut Grid, sensors: &Vec<Sensor>, y: isize) -> usize {
    // eliminate_squares(grid, sensors);
    let mut count = 0;
    // for col in grid.values() {
    //     match col.get(&y) {
    //         Some(GridState::NoBeacon) => count += 1,
    //         _ => (),
    //     }
    // }
    for x in -10_000_000..10_000_000 {
        if grid.get(&x).and_then(|c| c.get(&y)) == None {
            match sensors.iter().find(|s| s.is_in_range(x, y)) {
                Some(_) => count += 1,
                None => (),
            }
        }
    }

    count
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
        assert_eq!(sensors[3].beacon_x, 10);
        assert_eq!(sensors[3].beacon_y, 16);
        assert_eq!(sensors[0].beacon_x, -2);
    }

    #[test]
    fn test_eliminate_squares() {
        let (mut grid, sensors) = parse_file(TEST_STR);
        eliminate_squares(&mut grid, &sensors);
        assert_eq!(grid[&6][&0], GridState::NoBeacon);
        assert_eq!(grid[&15][&7], GridState::NoBeacon);
    }

    #[test]
    fn test_count_eliminated() {
        let (mut grid, sensors) = parse_file(TEST_STR);
        assert_eq!(count_eliminated(&mut grid, &sensors, 10), 26);
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
    // let contents = read_file(module_path!());
    // let (mut grid, sensors) = parse_file(&contents);
    // add_floor(&mut grid);
    // println!("{}", drop_sand(&mut grid));
}

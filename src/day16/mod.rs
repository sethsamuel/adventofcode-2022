use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use crate::file::read_file;
use indicatif::ProgressBar;
use itertools::Itertools;
use rayon::prelude::{ParallelBridge, ParallelIterator};
use regex::Regex;

#[derive(Debug, Default, Clone)]
struct Valve<'a> {
    id: &'a str,
    outs: HashSet<&'a str>,
    paths: HashMap<&'a str, Vec<&'a str>>,
    is_open: bool,
    rate: usize,
}

#[derive(Debug, Default, Clone)]
struct ValveGraph<'a> {
    valves: HashMap<&'a str, Valve<'a>>,
    pressure_released: usize,
    minutes_left: u8,
    current_valve: &'a str,
}

impl<'a> ValveGraph<'a> {
    fn add_valve(&mut self, valve: Valve<'a>) {
        self.valves.insert(valve.id, valve);
    }

    fn tick(&mut self) {
        if self.minutes_left == 0 {
            return;
        }
        self.valves
            .values()
            .filter(|v| v.is_open)
            .for_each(|v| self.pressure_released += v.rate);

        self.minutes_left -= 1
    }

    fn calculate_all_paths(&mut self) {
        let keys: Vec<&str> = self.valves.keys().map(|k| *k).collect();
        for valve in keys {
            self.calculate_paths(valve)
        }
    }

    fn calculate_paths(&mut self, valve: &str) {
        let keys: Vec<&str> = self.valves.keys().map(|k| *k).collect();
        let mut paths: HashMap<&str, Vec<&str>> =
            HashMap::from_iter(keys.iter().map(|k| (*k, Vec::new())));

        let mut queue: Vec<&str> = vec![valve];
        while let Some(current) = queue.pop() {
            let current_path = paths[current].clone();
            let outs = self.valves.get(current).unwrap().outs.clone();
            for out in outs {
                if paths.get(out).unwrap().len() == 0
                    || paths.get(out).unwrap().len() > current_path.len() + 1
                {
                    let mut path = current_path.clone();
                    path.push(out);
                    paths.insert(out, path);
                    queue.push(out);
                }
            }
        }
        self.valves.get_mut(valve).unwrap().paths = paths;
    }

    fn get_path_value(&self, from: &str, to: &str) -> isize {
        let mut value = 0;
        let path = self.valves.get(from).unwrap().paths.get(to).unwrap();
        for v in path {
            value += self.valves.get(v).unwrap().rate as isize;
        }
        value - (path.len() as isize * 2)
    }
    // fn get_expected_pressure(&self, from: &str, to: &str, minutes: u8) -> usize {
    //     let path_length = if from == to {
    //         0
    //     } else {
    //         self.valves.get(from).unwrap().paths.get(to).unwrap().len()
    //     };
    //     if (minutes as usize) <= path_length {
    //         return 0;
    //     }
    //     (minutes as usize - path_length - 1) * self.valves.get(to).unwrap().rate
    // }

    // fn get_best_pressure_path(&mut self, from: &str, minutes: u8) -> Vec<&str> {
    //     let mut best_pressure = 0;
    //     let mut best_pressure_valve = "";

    //     for to in self.valves.keys() {
    //         if self.valves.get(to).unwrap().is_open {
    //             continue;
    //         }
    //         let pressure = self.get_expected_pressure(from, to, minutes);
    //         // println!("Current best {}", best_pressure);
    //         // println!("Pressure {} for valve {}", pressure, to);
    //         if pressure > best_pressure {
    //             best_pressure = pressure;
    //             best_pressure_valve = *to;
    //         }
    //     }

    //     if best_pressure_valve == from {
    //         Vec::new()
    //     } else {
    //         self.valves
    //             .get(from)
    //             .unwrap()
    //             .paths
    //             .get(best_pressure_valve)
    //             .unwrap()
    //             .deref()
    //             .to_vec()
    //     }
    // }

    // fn execute(&mut self) -> usize {
    //     let mut current_valve = "AA";
    //     self.minutes_left = 30;

    //     // let mut clone = self.clone();
    //     while self.minutes_left > 0 {
    //         let best_path = {
    //             let ref this = self;
    //             let minutes = self.minutes_left;
    //             let mut best_pressure = 0;
    //             let mut best_pressure_valve = "";

    //             for to in this
    //                 .valves
    //                 .keys()
    //                 .filter(|v| this.valves.get(**v).unwrap().rate > 0)
    //             {
    //                 if this.valves.get(to).unwrap().is_open {
    //                     continue;
    //                 }
    //                 let pressure = this.get_expected_pressure(current_valve, to, minutes);
    //                 // println!("Current best {}", best_pressure);
    //                 // println!("Pressure {} for valve {}", pressure, to);
    //                 if pressure > best_pressure {
    //                     best_pressure = pressure;
    //                     best_pressure_valve = *to;
    //                 }
    //             }

    //             if best_pressure_valve == "" || best_pressure_valve == current_valve {
    //                 Vec::new()
    //             } else {
    //                 this.valves
    //                     .get(current_valve)
    //                     .unwrap()
    //                     .paths
    //                     .get(best_pressure_valve)
    //                     .unwrap()
    //                     .deref()
    //                     .to_vec()
    //             }
    //         };
    //         if best_path.len() == 0 {
    //             break;
    //         }
    //         let to_valve = best_path.last().unwrap();
    //         // println!("Minutes left {} path {:?}", self.minutes_left, best_path);
    //         for _ in 0..best_path.len() {
    //             self.tick();
    //         }
    //         self.tick();
    //         self.valves.get_mut(to_valve).unwrap().is_open = true;
    //         // clone.valves.get_mut(to_valve).unwrap().is_open = true;
    //         current_valve = to_valve;
    //     }

    //     self.pressure_released
    // }

    fn calculate_permutations(&self) -> Vec<Vec<&str>> {
        let non_zero_valves: Vec<&str> = self
            .valves
            .keys()
            .filter(|k| self.valves.get(*k).unwrap().rate > 0)
            .map(|k| *k)
            .collect();
        // println!("{:?}", non_zero_valves);
        let permutations = non_zero_valves
            .iter()
            .permutations(non_zero_valves.len())
            .map(|p| p.iter().map(|p| **p).collect())
            .collect();

        permutations
    }

    fn execute(&mut self, sequence: &Vec<&str>) -> usize {
        // let mut minutes_left = 30;
        // let mut pressure_released = 0;
        // let mut current_valve = "AA";
        // println!("Executing {:?}", sequence);
        self.minutes_left = 30;
        self.pressure_released = 0;
        self.current_valve = "AA";
        for valve in self.valves.iter_mut() {
            valve.1.is_open = false;
        }

        let mut to_valves = sequence.iter();
        while self.minutes_left > 0 && to_valves.len() > 0 {
            let next_valve = to_valves.next().unwrap();
            let path = self
                .valves
                .get(self.current_valve)
                .unwrap()
                .paths
                .get(next_valve)
                .unwrap()
                .clone();
            for _ in 0..path.len() {
                self.tick()
            }
            self.tick();
            self.current_valve = path.last().unwrap();
            self.valves.get_mut(self.current_valve).unwrap().is_open = true;
        }
        while self.minutes_left > 0 {
            self.tick();
        }

        // println!("{}", self.pressure_released);

        self.pressure_released
    }
}

fn parse_file(text: &str) -> ValveGraph {
    let regex = Regex::new(
        r"Valve (?P<valve>([A-Z])+) has flow rate=(?P<rate>(\d)+); tunnels? leads? to valves? (?P<out>[A-Z, ]+)",
    ).unwrap();

    let mut graph = ValveGraph::default();
    text.lines().for_each(|l| {
        let captures = regex.captures(l).unwrap();
        let outs = captures.name("out").unwrap().as_str().split(", ");
        graph.add_valve(Valve {
            id: captures.name("valve").unwrap().as_str(),
            outs: HashSet::from_iter(outs),
            paths: HashMap::new(),
            is_open: false,
            rate: captures.name("rate").unwrap().as_str().parse().unwrap(),
        });
    });

    graph
}

#[cfg(test)]
mod tests {
    use rayon::prelude::IntoParallelIterator;

    use super::*;

    static TEST_STR: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_parse_file() {
        let graph = parse_file(TEST_STR);
        assert_eq!(graph.valves.len(), 10);
        assert_eq!(graph.valves["GG"].outs.len(), 2);
        assert!(graph.valves["GG"].outs.contains("HH"));
    }

    #[test]
    fn test_calculate_paths() {
        let mut graph = parse_file(TEST_STR);
        graph.calculate_paths("AA");
        assert_eq!(
            graph.valves.get("AA").unwrap().paths.get("DD").unwrap(),
            &vec!["DD"]
        );
        assert_eq!(
            graph.valves.get("AA").unwrap().paths.get("FF").unwrap(),
            &vec!["DD", "EE", "FF"]
        );
    }

    #[test]
    fn test_calculate_all_paths() {
        let mut graph = parse_file(TEST_STR);
        graph.calculate_all_paths();
        assert_eq!(
            graph.valves.get("AA").unwrap().paths.get("DD").unwrap(),
            &vec!["DD"]
        );
        assert_eq!(
            graph.valves.get("AA").unwrap().paths.get("FF").unwrap(),
            &vec!["DD", "EE", "FF"]
        );
        assert_eq!(
            graph.valves.get("FF").unwrap().paths.get("AA").unwrap(),
            &vec!["EE", "DD", "AA"]
        );
    }

    #[test]
    fn test_get_expected_pressure() {
        let mut graph = parse_file(TEST_STR);
        graph.calculate_all_paths();
        for v in graph.valves.keys() {
            println!(
                "{} {} {:?}",
                v,
                graph.get_path_value("AA", v),
                graph.valves.get("AA").unwrap().paths.get(v).unwrap()
            );
        }
        println!("DD");
        for v in graph.valves.keys() {
            println!(
                "{} {} {:?}",
                v,
                graph.get_path_value("BB", v),
                graph.valves.get("BB").unwrap().paths.get(v).unwrap()
            );
        }
        // assert_eq!(graph.get_expected_pressure("AA", "DD", 30), (30 - 2) * 20);
        // assert_eq!(graph.get_expected_pressure("AA", "BB", 30), 364);
        // assert_eq!(graph.get_expected_pressure("AA", "CC", 30), 54);
    }

    // #[test]
    // fn test_get_best_pressure_path() {
    //     let mut graph = parse_file(TEST_STR);
    //     graph.calculate_all_paths();
    //     assert_eq!(graph.get_best_pressure_path("AA", 30), vec!["DD"]);
    // }

    #[test]
    fn test_execute() {
        let mut graph = parse_file(TEST_STR);
        graph.calculate_all_paths();
        assert_eq!(
            graph
                .clone()
                .execute(&vec!["DD", "BB", "JJ", "HH", "EE", "CC"]),
            1651
        );
        let permutations = graph.calculate_permutations();
        // println!("{:?}", permutations);
        let output: usize = permutations
            .into_par_iter()
            .fold(
                || 0,
                |mut acc, p| {
                    let result = graph.clone().execute(&p);
                    acc = result.max(acc);
                    acc
                },
            )
            .max()
            .unwrap();
        // print!("{:?}", outputs);
        assert_eq!(output, 1651);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let mut graph = parse_file(&contents);
    graph.calculate_all_paths();
    let non_zero_valves: Vec<&str> = graph
        .valves
        .keys()
        .filter(|k| graph.valves.get(*k).unwrap().rate > 0)
        .map(|k| *k)
        .collect();
    // println!("{:?}", non_zero_valves);
    // let start = vec!["OM", "RO", "SP"];

    let permutations = non_zero_valves.iter().permutations(8);
    let output: usize = permutations
        .par_bridge()
        .fold(
            || 0,
            |mut acc, p| {
                let result = graph
                    .clone()
                    .execute(&p.iter().map(|s| **s).collect::<Vec<&str>>());
                if result.max(acc) > acc {
                    println!("{} {:?}", result, p)
                }
                acc = result.max(acc);
                acc
            },
        )
        .max()
        .unwrap();

    println!("{}", output);
    // let mut highest_result = 0;

    // for i in 1..=5 {
    //     // for s in non_zero_valves.iter() {
    //     // let start = vec!["OM", "VR", "RO"];
    //     // let start = vec!["OM", "VR", "RO", "SP", "KZ", "DI", "SO"];
    //     // let start = vec!["OM", "RO", "SP", "KZ", "DI", "SO", "SC"];
    //     // let start = vec!["RO", "SP", "KZ", "DI", "SO", "OM"];
    //     // let start = vec!["RO", "SP", "KZ", "DI", "SO", "SC", "PW", "IR", "OM", "RI"];
    //     let start = vec!["SP", "KZ"];
    //     // let start = vec![];
    //     // let start = vec![*s];
    //     let permutations = non_zero_valves
    //         .iter()
    //         .filter(|v| !start.contains(*v))
    //         .permutations(i);

    //     let bar = ProgressBar::new(360360 as u64);
    //     for p in permutations {
    //         bar.inc(1);
    //         let mut end = p.iter().map(|s| **s).collect::<Vec<&str>>();

    //         {
    //             let mut path = start.clone();
    //             path.append(&mut end.clone());
    //             let result = graph.clone().execute(&path);

    //             if result > highest_result {
    //                 println!("{result}, {:?}", path);
    //                 highest_result = result;
    //             }
    //         }
    //         {
    //             let mut path = end.clone();
    //             path.append(&mut start.clone());
    //             let result = graph.clone().execute(&path);

    //             if result > highest_result {
    //                 println!("{result}, {:?}", path);
    //                 highest_result = result;
    //             }
    //         }
    //         // result
    //     }
    //     bar.finish();
    // }
    // }
    // println!("{}", outputs.max().unwrap());
}

#[allow(dead_code)]
pub fn part2() {
    // let contents = read_file(module_path!());
    // let (mut grid, sensors) = parse_file(&contents);
    // println!(
    //     "{}",
    //     find_tuning(&mut grid, &sensors, 4_000_000, 4_000_000).unwrap()
    // );
}

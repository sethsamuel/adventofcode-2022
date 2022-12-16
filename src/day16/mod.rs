use std::collections::{HashMap, HashSet};

use crate::file::read_file;
use regex::Regex;

#[derive(Debug, Default)]
struct Valve<'a> {
    id: &'a str,
    paths: HashSet<&'a str>,
}

#[derive(Debug, Default)]
struct ValveGraph<'a> {
    valves: HashMap<&'a str, Valve<'a>>,
}

impl<'a> ValveGraph<'a> {
    fn add_valve(&mut self, valve: Valve<'a>) {
        self.valves.insert(valve.id, valve);
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
            paths: HashSet::from_iter(outs),
        });
    });

    graph
}

#[cfg(test)]
mod tests {
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
        assert_eq!(graph.valves["GG"].paths.len(), 2);
        assert!(graph.valves["GG"].paths.contains("HH"));
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
    // let contents = read_file(module_path!());
    // let (mut grid, sensors) = parse_file(&contents);
    // println!(
    //     "{}",
    //     find_tuning(&mut grid, &sensors, 4_000_000, 4_000_000).unwrap()
    // );
}

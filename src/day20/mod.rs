use std::{collections::HashMap, ops::Deref};

use crate::file::read_file;
#[derive(Debug, Default)]
struct Array {
    left_links: HashMap<isize, isize>,
    right_links: HashMap<isize, isize>,
}

impl Array {
    fn insert(&mut self, value: isize, left: isize, right: isize) {
        self.left_links.insert(value, left);
        self.right_links.insert(value, right);
    }

    fn shift(&mut self, value: isize) {
        let direction = value.signum();
        for _ in 0..value.abs() {
            if direction == 1 {
                let right = *self.right_links.get(&value).unwrap();
                // println!("right of {value} is {right}");
                self.left_links
                    .insert(right, *self.left_links.get(&value).unwrap());
                self.right_links
                    .insert(*self.left_links.get(&value).unwrap(), right);
                self.left_links.insert(value, right);
                self.right_links
                    .insert(value, *self.right_links.get(&right).unwrap());
                self.left_links
                    .insert(*self.right_links.get(&right).unwrap(), value);
                self.right_links.insert(right, value);
            } else if direction == -1 {
                let left = *self.left_links.get(&value).unwrap();
                self.right_links
                    .insert(left, *self.right_links.get(&value).unwrap());
                self.left_links
                    .insert(*self.right_links.get(&value).unwrap(), left);
                self.right_links.insert(value, left);
                self.left_links
                    .insert(value, *self.left_links.get(&left).unwrap());
                self.right_links
                    .insert(*self.left_links.get(&left).unwrap(), value);
                self.left_links.insert(left, value);
            }
        }
    }

    fn get_offset(&self, value: isize, offset: usize) -> isize {
        let mut result = value;
        for _ in 0..offset {
            result = *self.right_links.get(&result).unwrap();
        }
        result
    }
}

fn parse_file(text: &str) -> (Array, Vec<isize>) {
    let mut array = Array {
        ..Default::default()
    };

    let vec: Vec<isize> = text.lines().map(|l| l.parse::<isize>().unwrap()).collect();
    array.insert(vec[0], *vec.last().unwrap(), vec[1]);
    array.insert(*vec.last().unwrap(), vec[vec.len() - 2], vec[0]);
    for i in 1..vec.len() - 1 {
        array.insert(vec[i], vec[i - 1], vec[i + 1]);
    }
    (array, vec)
}

fn mix(array: &mut Array, vec: &Vec<isize>) {
    for v in vec.iter() {
        array.shift(*v);
        // println!("{:?}", array);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_parse_file() {
        let (array, vec) = parse_file(TEST_STR);
        assert_eq!(*array.left_links.get(&-2).unwrap(), 3);
        assert_eq!(vec[2], -3);
    }

    #[test]
    fn test_mix() {
        let (mut array, vec) = parse_file(TEST_STR);
        mix(&mut array, &vec);
        assert_eq!(*array.left_links.get(&4).unwrap(), -3);
        assert_eq!(*array.right_links.get(&0).unwrap(), 3);
    }

    #[test]
    fn test_get_offset() {
        let (mut array, vec) = parse_file(TEST_STR);
        assert_eq!(array.get_offset(0, 1), 4);
        assert_eq!(array.get_offset(0, 3), 2);
        mix(&mut array, &vec);
        assert_eq!(array.get_offset(0, 1000), 4);
        assert_eq!(array.get_offset(0, 2000), -3);
        assert_eq!(array.get_offset(0, 3000), 2);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let (mut array, vec) = parse_file(&contents);
    mix(&mut array, &vec);
    println!(
        "{}",
        array.get_offset(0, 1000) + array.get_offset(0, 2000) + array.get_offset(0, 3000)
    );
    // assert_eq!(array.get_offset(0, 1000), 4);
    // assert_eq!(array.get_offset(0, 2000), -3);
    // assert_eq!(array.get_offset(0, 3000), 2);}
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

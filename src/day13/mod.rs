use std::cmp::Ordering;

use crate::file::read_file;
#[derive(Debug, Clone)]
struct Array {
    items: Option<Vec<Array>>,
    number: Option<u32>,
}

impl ToString for Array {
    fn to_string(&self) -> String {
        if let Some(items) = &self.items {
            return format!(
                "[{}]",
                items
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            );
        } else if let Some(n) = self.number {
            return n.to_string();
        } else {
            return "".to_string();
        }
    }
}

// impl DerefMut for Array {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.items.unwrap()
//     }
// }

impl PartialOrd for Array {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.number {
            Some(l) => match other.number {
                Some(r) => l.partial_cmp(&r),
                None => match other.items.clone() {
                    Some(_) => Array::from(vec![l]).partial_cmp(other),
                    None => Some(Ordering::Greater),
                },
            },
            None => match self.items.clone() {
                Some(l_items) => match other.items.clone() {
                    Some(r_items) => {
                        for i in 0..=(l_items.len().max(r_items.len())) {
                            if i >= l_items.len() && i >= r_items.len() {
                                return Some(Ordering::Equal);
                            }
                            if i >= l_items.len() {
                                return Some(Ordering::Less);
                            }
                            if i >= r_items.len() {
                                return Some(Ordering::Greater);
                            }
                            if l_items[i] > r_items[i] {
                                return Some(Ordering::Greater);
                            }
                            if l_items[i] < r_items[i] {
                                return Some(Ordering::Less);
                            }
                        }
                        Some(Ordering::Equal)
                    }
                    None => match other.number {
                        Some(r) => self.partial_cmp(&Array::from(vec![r])),
                        None => Some(Ordering::Greater),
                    },
                },
                None => Some(Ordering::Less),
            },
        }
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        )
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
        )
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Equal))
    }
}

impl From<u32> for Array {
    fn from(n: u32) -> Self {
        Array {
            items: None,
            number: Some(n),
        }
    }
}

impl From<&u32> for Array {
    fn from(n: &u32) -> Self {
        <u32 as Into<Array>>::into(*n)
    }
}

impl From<Vec<u32>> for Array {
    fn from(items: Vec<u32>) -> Self {
        Array {
            items: Some(items.iter().map(|i| i.into()).collect::<Vec<Array>>()),
            number: None,
        }
    }
}

impl From<Vec<Array>> for Array {
    fn from(items: Vec<Array>) -> Self {
        Array {
            items: Some(items),
            number: None,
        }
    }
}

type Pair = (Array, Array);
trait IsValid {
    fn is_valid(&self) -> bool;
}

impl IsValid for Pair {
    fn is_valid(&self) -> bool {
        self.0 < self.1
    }
}

fn parse_line(text: &str) -> Array {
    let mut chars = text.chars();
    let mut char_stack: Vec<char> = Vec::new();
    let mut array_stack: Vec<Array> = Vec::new();
    let mut last_array: Array = Array {
        items: None,
        number: None,
    };
    while let Some(c) = chars.next() {
        // println!("{:?}", c);
        // println!("{:?}", array_stack);
        // println!("{:?}", char_stack);
        match c {
            '[' => array_stack.push(Array {
                items: Some(Vec::new()),
                number: None,
            }),
            ']' => {
                if let Some(number) = char_stack.iter().collect::<String>().parse::<u32>().ok() {
                    char_stack.truncate(0);
                    if let Some(last_array) = array_stack.last_mut() {
                        if let Some(items) = last_array.items.as_mut() {
                            items.push(Array {
                                items: None,
                                number: Some(number),
                            });
                        }
                    }
                }

                let array = array_stack.pop().unwrap().clone();
                last_array = array.clone();
                if let Some(last_array) = array_stack.last_mut() {
                    if let Some(items) = last_array.items.as_mut() {
                        items.push(Array::from(array));
                    }
                }
            }
            ',' => {
                if char_stack.len() > 0 {
                    let number = char_stack
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    char_stack.truncate(0);

                    if let Some(last_array) = array_stack.last_mut() {
                        if let Some(items) = (last_array).items.as_mut() {
                            items.push(Array {
                                items: None,
                                number: Some(number),
                            });
                        }
                    }
                } else {
                }
            }
            d => char_stack.push(d),
        }
    }

    // println!("{:?}", last_array);

    last_array
}

fn parse_file(text: &str) -> Vec<Pair> {
    text.split("\n\n")
        .map(|p| {
            let mut lines = p.split("\n");
            (
                parse_line(lines.next().unwrap()),
                parse_line(lines.next().unwrap()),
            )
        })
        .collect()
}

fn get_valid_pairs(pairs: &Vec<Pair>) -> usize {
    let mut sum = 0;
    for i in 0..pairs.len() {
        if pairs[i].is_valid() {
            sum += i + 1;
        }
    }
    sum
}

fn get_packets(pairs: &Vec<Pair>) -> Vec<Array> {
    let mut packets = Vec::new();
    for pair in pairs {
        packets.push(pair.0.clone());
        packets.push(pair.1.clone());
    }

    packets.push(vec![Array::from(vec![2])].into());
    packets.push(vec![Array::from(vec![6])].into());

    packets
}

fn get_sorted_packets(pairs: &Vec<Pair>) -> Vec<Array> {
    let mut packets = get_packets(pairs);
    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // for p in packets.iter() {
    //     println!("{}", p.to_string());
    // }
    packets
}

fn get_decoder_key(pairs: &Vec<Pair>) -> usize {
    let packets = get_sorted_packets(pairs);
    let index1 = packets
        .iter()
        .position(|a| a.to_string() == "[[2]]")
        .unwrap()
        + 1;
    let index2 = packets
        .iter()
        .position(|a| a.to_string() == "[[6]]")
        .unwrap()
        + 1;

    index1 * index2
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_parse_file() {
        let pairs = parse_file(TEST_STR);
        assert_eq!(pairs.len(), 8);
        assert_eq!(pairs[0].0.items.as_ref().unwrap().len(), 5);
        assert_eq!(pairs[0].1.items.as_ref().unwrap().len(), 5);
        assert_eq!(pairs[1].0.items.as_ref().unwrap().len(), 2);
        assert_eq!(pairs[1].1.items.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_compare() {
        assert!((vec![1, 1, 3, 1, 1].into(), vec![1, 1, 5, 1, 1].into()).is_valid());
        assert!((
            vec![Array::from(vec![1]), Array::from(vec![2, 3, 4])].into(),
            vec![Array::from(vec![1]), Array::from(4)].into(),
        )
            .is_valid());
        assert!(!(vec![9].into(), vec![Array::from(vec![8, 7, 6])].into()).is_valid());
        assert!(!(
            vec![Array::from(vec![Array::from(vec![] as Vec<u32>)])].into(),
            vec![Array::from(vec![] as Vec<u32>)].into()
        )
            .is_valid());
    }

    #[test]
    fn test_get_valid_pairs() {
        let pairs = parse_file(TEST_STR);
        assert_eq!(get_valid_pairs(&pairs), 13);
    }

    #[test]
    fn test_get_decoder_key() {
        let pairs = parse_file(TEST_STR);
        assert_eq!(get_decoder_key(&pairs), 140);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let pairs = parse_file(&contents);

    println!("{}", get_valid_pairs(&pairs));
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let pairs = parse_file(&contents);
    println!("{}", get_decoder_key(&pairs));
}

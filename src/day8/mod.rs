use crate::file::read_file;

type Forrest = Vec<Vec<u8>>;

fn parse_file(text: &str) -> Forrest {
    let forrest: Forrest = text
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    forrest
}

fn get_row(row: usize, forrest: &Forrest) -> Vec<u8> {
    forrest[row].clone()
}

fn get_column(column: usize, forrest: &Forrest) -> Vec<u8> {
    forrest.iter().map(|r| r[column]).collect()
}

fn is_visible(row: usize, column: usize, forrest: &Forrest) -> bool {
    let forrest_row = get_row(row, forrest);
    let forrest_column = get_column(column, forrest);
    let tree = forrest[row][column];
    forrest_row[0..column].iter().max().lt(&Some(&tree))
        || forrest_row[column + 1..].iter().max().lt(&Some(&tree))
        || forrest_column[0..row].iter().max().lt(&Some(&tree))
        || forrest_column[row + 1..].iter().max().lt(&Some(&tree))
}

fn count_visible_trees(forrest: &Forrest) -> usize {
    let mut visible_count = 0;
    for row in 0..forrest.len() {
        for column in 0..forrest[row].len() {
            if is_visible(row, column, forrest) {
                // println!("visible {} {}", row, column);
                visible_count += 1;
            }
        }
    }
    visible_count
}

fn get_scenic_score(row: usize, column: usize, forrest: &Forrest) -> usize {
    let tree = forrest[row][column];
    let forrest_row = get_row(row, forrest);
    let forrest_column = get_column(column, forrest);

    let mut left: usize = 0;
    for i in 1..=column {
        left += 1;
        if forrest_row[column - i] >= tree {
            break;
        }
    }

    let mut right: usize = 0;
    for i in (column + 1)..forrest_row.len() {
        right += 1;
        if forrest_row[i] >= tree {
            break;
        }
    }

    let mut up: usize = 0;
    for i in 1..=row {
        up += 1;
        if forrest_column[row - i] >= tree {
            break;
        }
    }

    let mut down: usize = 0;
    for i in (row + 1)..forrest_column.len() {
        down += 1;
        if forrest_column[i] >= tree {
            break;
        }
    }

    left * right * up * down
}

fn get_most_scenic(forrest: &Forrest) -> usize {
    let mut max = 0;
    for row in 0..forrest.len() {
        for column in 0..forrest[row].len() {
            let score = get_scenic_score(row, column, forrest);
            if score > max {
                max = score;
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_parse_file() {
        let forrest = parse_file(TEST_STR);
        assert_eq!(forrest.len(), 5);
        assert_eq!(forrest[0][3], 7);
    }

    #[test]
    fn test_is_visible() {
        let forrest = parse_file(TEST_STR);
        assert!(is_visible(0, 0, &forrest));
        assert!(is_visible(0, 3, &forrest));
        assert!(is_visible(0, 4, &forrest));
        assert!(!is_visible(1, 3, &forrest));
        assert!(is_visible(2, 3, &forrest));
        assert!(is_visible(1, 1, &forrest));
        assert!(is_visible(4, 2, &forrest));
        assert!(is_visible(4, 4, &forrest));
    }

    #[test]
    fn test_count_visible_trees() {
        let forrest = parse_file(TEST_STR);
        assert_eq!(count_visible_trees(&forrest), 21);
    }

    #[test]
    fn test_get_scenic_score() {
        let forrest = parse_file(TEST_STR);
        assert_eq!(get_scenic_score(1, 2, &forrest), 4);
        assert_eq!(get_scenic_score(3, 2, &forrest), 8);
    }

    #[test]
    fn test_get_most_scenic() {
        let forrest = parse_file(TEST_STR);
        assert_eq!(get_most_scenic(&forrest), 8);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let forrest = parse_file(&contents);

    println!("{:?}", count_visible_trees(&forrest));
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let forrest = parse_file(&contents);

    println!("{:?}", get_most_scenic(&forrest));
}

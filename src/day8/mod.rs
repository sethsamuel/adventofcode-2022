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

fn is_offset_visible(row: usize, column: usize, vector: (isize, isize), forrest: &Forrest) -> bool {
    is_visible(
        (row as isize + vector.0) as usize,
        (column as isize + vector.1) as usize,
        Some(vector),
        forrest,
    )
}

fn is_visible(
    row: usize,
    column: usize,
    vector: Option<(isize, isize)>,
    forrest: &Forrest,
) -> bool {
    // println!("is visible? {} {}", row, column);
    if row == 0 || column == 0 {
        // println!("Edge");
        return true;
    }
    if row == forrest.len() - 1 || column == forrest[row].len() - 1 {
        // println!("Edge");
        return true;
    }
    match vector {
        Some((r, c)) => {
            is_offset_visible(row, column, (r, c), forrest)
                && forrest[row][column]
                    >= forrest[(row as isize + r) as usize][(column as isize + c) as usize]
        }
        _ => {
            (is_offset_visible(row, column, (-1, 0), forrest)
                && forrest[row][column]
                    > forrest[(row as isize + (-1)) as usize][(column as isize + (0)) as usize])
                || (is_offset_visible(row, column, (1, 0), forrest)
                    && forrest[row][column]
                        > forrest[(row as isize + (1)) as usize][(column as isize + (0)) as usize])
                || (is_offset_visible(row, column, (0, -1), forrest)
                    && forrest[row][column]
                        > forrest[(row as isize + (0)) as usize][(column as isize + (-1)) as usize])
                || (is_offset_visible(row, column, (0, 1), forrest)
                    && forrest[row][column]
                        > forrest[(row as isize + (0)) as usize][(column as isize + (1)) as usize])
        }
    }
}

fn count_visible_trees(forrest: &Forrest) -> usize {
    let mut visible_count = 0;
    for row in 0..forrest.len() {
        for column in 0..forrest[row].len() {
            if is_visible(row, column, None, forrest) {
                // println!("visible {} {}", row, column);
                visible_count += 1;
            }
        }
    }
    visible_count
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
        assert!(is_visible(0, 0, None, &forrest));
        assert!(is_visible(0, 3, None, &forrest));
        assert!(is_visible(0, 4, None, &forrest));
        assert!(!is_visible(1, 3, None, &forrest));
        assert!(is_visible(2, 3, None, &forrest));
        assert!(is_visible(1, 1, None, &forrest));
        assert!(is_visible(4, 2, None, &forrest));
    }

    #[test]
    fn test_count_visible_trees() {
        let forrest = parse_file(TEST_STR);
        assert_eq!(count_visible_trees(&forrest), 21);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let forrest = parse_file(&contents);
    // let large = get_large_directories(&files);

    println!("{:?}", count_visible_trees(&forrest));
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let files = parse_file(&contents);
    // let dir = get_smallest_deletable_directory(&files);
    // println!("{:?}", dir);
}

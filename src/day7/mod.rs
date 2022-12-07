use crate::file::read_file;

#[derive(Debug)]
struct FileSystem<'a> {
    nodes: Vec<File<'a>>,
}

impl<'a> FileSystem<'a> {
    fn add_file(&mut self, name: &'a str, size: usize, parent: Option<usize>) -> usize {
        let index = self.nodes.len();
        let node = File {
            id: index,
            name,
            size,
            children: Vec::new(),
            parent,
        };
        self.nodes.push(node);
        match parent {
            Some(i) => self.nodes[i].children.push(index),
            None => (),
        }

        index
    }

    fn total_size(&self, file: usize) -> usize {
        match self.nodes[file].size {
            0 => self.nodes[file]
                .children
                .iter()
                .map(|c| self.total_size(*c))
                .sum(),
            size => size,
        }
    }

    fn print(&self) {
        self.print_file(0, 0)
    }

    fn print_file(&self, file_handle: usize, depth: usize) {
        let file = &self.nodes[file_handle];
        println!(
            "{}{} ({})",
            (0..depth).map(|_| " ").collect::<String>(),
            file.name,
            self.total_size(file_handle)
        );
        file.children.iter().for_each(|c| {
            self.print_file(*c, depth + 1);
        })
    }
}

#[derive(Debug)]
struct File<'a> {
    id: usize,
    name: &'a str,
    size: usize,
    children: Vec<usize>,
    parent: Option<usize>,
}

impl<'a> File<'a> {}

fn parse_file(text: &str) -> FileSystem {
    let mut files = FileSystem { nodes: Vec::new() };
    let mut current_file = files.add_file(&"/", 0, None);
    let lines = text.lines();
    lines.for_each(|l| -> () {
        if l.starts_with("$") {
            let mut commands = l[2..].split(" ");
            match commands.next() {
                Some("ls") => (),
                Some("cd") => match commands.next() {
                    Some("/") => current_file = 0,
                    Some("..") => current_file = files.nodes[current_file].parent.unwrap_or(0),
                    Some(dir) => current_file = files.add_file(dir, 0, Some(current_file)),
                    _ => (),
                },
                _ => (),
            }
        } else {
            let mut parts = l.split(" ");
            match parts.next() {
                Some("dir") => (),
                Some(size) => match size.parse::<usize>() {
                    Ok(size) => {
                        let name = parts.next().unwrap();
                        files.add_file(name, size, Some(current_file));
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    });
    files
}

fn get_large_directories(files: &FileSystem) -> Vec<usize> {
    files
        .nodes
        .iter()
        .filter(|f| f.children.len() > 0 && files.total_size(f.id) <= 100000)
        .map(|f| f.id)
        .collect()
}

fn get_smallest_deletable_directory(files: &FileSystem) -> usize {
    let total_space: usize = 70000000;
    let used_space = files.total_size(0);
    let needed_space = 30000000 - (total_space - used_space);
    let mut smallest = usize::MAX;
    files.nodes.iter().for_each(|f| {
        if f.children.len() == 0 {
            return;
        }
        let file_size = files.total_size(f.id);
        if (needed_space..smallest).contains(&file_size) {
            smallest = file_size;
        }
    });
    smallest
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_parse_file() {
        let files = parse_file(TEST_STR);
        files.print()
    }

    #[test]
    fn test_get_large_directories() {
        let files = parse_file(TEST_STR);
        let large = get_large_directories(&files);
        assert_eq!(large.len(), 2);
        assert_eq!(
            large.iter().map(|f| files.total_size(*f)).sum::<usize>(),
            95437
        )
    }

    #[test]
    fn test_get_smallest_deletable_directory() {
        let files = parse_file(TEST_STR);
        let dir = get_smallest_deletable_directory(&files);
        assert_eq!(dir, 24933642);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let contents = read_file(module_path!());
    let files = parse_file(&contents);
    let large = get_large_directories(&files);

    println!(
        "{:?}",
        large.iter().map(|f| files.total_size(*f)).sum::<usize>()
    );
}

#[allow(dead_code)]
pub fn part2() {
    let contents = read_file(module_path!());
    let files = parse_file(&contents);
    let dir = get_smallest_deletable_directory(&files);
    println!("{:?}", dir);
}

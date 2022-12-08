use std::cell::RefCell;
use std::str::Lines;

const DISK_SIZE: u64 = 70_000_000;
const NEEDED_SPACE: u64 = 30_000_000;

#[derive(Clone, Default)]
struct Directory {
    directories: RefCell<Vec<Directory>>,
    size: u64,
}

impl Directory {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn total_size(&self) -> u64 {
        let dir_sizes = self
            .directories
            .borrow()
            .iter()
            .map(Self::total_size)
            .sum::<u64>();
        self.size + dir_sizes
    }
}

fn read_dir(parent_dir: &mut Directory, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        let tokens = line.split_whitespace().collect::<Vec<_>>();

        match &tokens[..] {
            &["$", "cd", dir_name] => {
                if dir_name == ".." {
                    return;
                }
                let mut child_dir = Directory::new();
                read_dir(&mut child_dir, lines);
                parent_dir.directories.borrow_mut().push(child_dir)
            }
            &["$", "ls"] => {}
            &["dir", _] => {}
            &[file_size, _file_name] => {
                let size: u64 = file_size.parse().expect("cannot parse size");

                parent_dir.size += size;
            }
            _ => (),
        }
    }
}

fn get_dir_sizes(parent_dir: &mut Directory) -> Vec<u64> {
    let mut sizes = vec![parent_dir.total_size()];

    for child_dir in parent_dir.directories.borrow_mut().iter_mut() {
        sizes.append(&mut get_dir_sizes(child_dir));
    }

    sizes
}

fn part1(parent_dir: &mut Directory) -> u64 {
    get_dir_sizes(parent_dir)
        .iter()
        .filter(|&&size| size <= 100_000)
        .sum::<u64>()
}

fn part2(parent_dir: &mut Directory) -> u64 {
    let sizes = get_dir_sizes(parent_dir);
    let unused_space = DISK_SIZE - &sizes[0];
    let min_space = NEEDED_SPACE - unused_space;

    *sizes.iter().filter(|&&size| size >= min_space).min().unwrap()
}

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    lines.next();
    let mut parent_dir = &mut Directory::new();

    read_dir(&mut parent_dir, &mut lines);
    let part1 = part1(parent_dir);
    let part2 = part2(parent_dir);

    println!("{part1}\n{part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../test-input.txt");

    #[test]
    fn test_part1() {
        let mut lines = INPUT.lines();
        lines.next();
        let mut parent_dir = &mut Directory::new();

        read_dir(&mut parent_dir, &mut lines);
        assert_eq!(95437, part1(parent_dir));
    }

    #[test]
    fn test_part2() {
        let mut lines = INPUT.lines();
        lines.next();
        let mut parent_dir = &mut Directory::new();

        read_dir(&mut parent_dir, &mut lines);
        assert_eq!(24933642, part2(parent_dir));
    }
}

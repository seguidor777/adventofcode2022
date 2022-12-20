use std::collections::{HashSet, VecDeque};

#[rustfmt::skip]
const DIRECTIONS: &'static [(isize, isize); 4] = &[
    (0, -1), // Up
    (0, 1),  // Down
    (-1, 0), // Left
    (1, 0)   // Right
];

type Pos = (usize, usize);
type Queue = VecDeque<(Node, Vec<Node>)>;
type Grid = Vec<Vec<Node>>;

fn build_grid(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, &b)| Node::new(b, x, y))
                .collect()
        })
        .collect()
}

fn find_pos(grid: &Grid, value: u8) -> Pos {
    let row_width = grid[0].len();

    (0..grid.len())
        .map(|y| (0..row_width).map(move |x| (x, y)))
        .flatten()
        .find(|&(x, y)| grid[y][x].value == value)
        .expect("cannot find node")
}

fn find_queue(grid: &Grid, value: u8) -> Queue {
    let row_width = grid[0].len();

    (0..grid.len())
        .map(|y| (0..row_width).map(move |x| (x, y)))
        .flatten()
        .filter(|&(x, y)| grid[y][x].value == value)
        .map(|(x, y)| (Node::new(grid[y][x].value, x, y), vec![]))
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Node {
    value: u8,
    x: usize,
    y: usize,
}

impl Node {
    fn new(value: u8, x: usize, y: usize) -> Self {
        Self { value, x, y }
    }

    fn neighbors(&self, grid: &[Vec<Node>]) -> Vec<Node> {
        DIRECTIONS
            .iter()
            .filter_map(|&(dx, dy)| {
                grid.get((self.y as isize + dy) as usize)
                    .and_then(|col| col.get((self.x as isize + dx) as usize))
            })
            .copied()
            .filter(|neighbor| neighbor.value <= self.value + 1)
            .collect()
    }
}

fn find_best_signal(grid: &Grid, mut queue: Queue, end: &Node) -> Option<Vec<Node>> {
    let mut visited: HashSet<Node> = HashSet::new();

    while let Some((current, steps)) = queue.pop_front() {
        if current.value == end.value {
            return Some(steps);
        }

        for neighbor in current.neighbors(grid) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                let mut steps = steps.clone();
                steps.push(neighbor);
                queue.push_back((neighbor, steps));
            }
        }
    }

    None
}

fn part1(input: &str) -> usize {
    let mut grid = build_grid(input);
    let (start_x, start_y) = find_pos(&grid, b'S');
    let (end_x, end_y) = find_pos(&grid, b'E');
    grid[start_y][start_x].value = b'a';
    grid[end_y][end_x].value = b'z';
    let mut queue = Queue::new();
    queue.push_back((grid[start_y][start_x], vec![]));

    find_best_signal(&grid, queue, &grid[end_y][end_x])
        .expect("goal not reached")
        .len()
}

fn part2(input: &str) -> usize {
    let mut grid = build_grid(input);
    let queue = find_queue(&grid, b'a');
    let (start_x, start_y) = find_pos(&grid, b'S');
    let (end_x, end_y) = find_pos(&grid, b'E');

    grid[start_y][start_x].value = b'a';
    grid[end_y][end_x].value = b'z';

    find_best_signal(&grid, queue, &grid[end_y][end_x])
        .expect("goal not reached")
        .len()
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input);
    let part2 = part2(input);

    println!("{part1}\n{part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../test-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(31, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(29, part2(INPUT));
    }
}

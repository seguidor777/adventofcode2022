use std::borrow::BorrowMut;

fn is_visible(grid: &Vec<Vec<u8>>, x: &usize, y: &usize, height: &u8) -> bool {
    // Is on the edge
    if *x == grid[0].len().saturating_sub(1) || *y == grid.len().saturating_sub(1) {
        return true;
    }

    // Is taller than trees between
    grid[..*y].iter().rev().all(|row| *height > row[*x])                      // Up
        || grid[*y][..*x].iter().rev().all(|&other_height| *height > other_height) // Left
        || grid[*y][*x+1..].iter().all(|&other_height| *height > other_height)     // Right
        || grid[y+1..].iter().all(|row| *height > row[*x])                    // Down
}

fn part1(grid: &Vec<Vec<u8>>) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, height)| {
                is_visible(&grid, &x, &y, height).then_some((x, y, height))
            })
        })
        .count()
}

fn calculate_scenic_score(grid: &Vec<Vec<u8>>, x: &usize, y: &usize, height: &u8) -> usize {
    let (max_x, max_y) = (grid[0].len(), grid.len());
    let mut up_cursor = (0..*y).rev();
    let mut up = up_cursor
        .borrow_mut()
        .take_while(|&y| height > &grid[y][*x])
        .count();
    if up_cursor.next().is_some() {
        up += 1
    }
    let mut left_cursor = (0..*x).rev();
    let mut left = left_cursor
        .borrow_mut()
        .take_while(|&x| height > &grid[*y][x])
        .count();
    if left_cursor.next().is_some() {
        left += 1
    }
    let mut right_cursor = *x + 1..max_x;
    let mut right = right_cursor
        .borrow_mut()
        .take_while(|&x| height > &grid[*y][x])
        .count();
    if right_cursor.next().is_some() {
        right += 1
    }
    let mut down_cursor = y + 1..max_y;
    let mut down = down_cursor
        .borrow_mut()
        .take_while(|&y| height > &grid[y][*x])
        .count();
    if down_cursor.next().is_some() {
        down += 1
    }

    [up, left, right, down]
        .iter()
        .filter(|&&score| score > 0)
        .product()
}

fn part2(grid: &Vec<Vec<u8>>) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, height)| calculate_scenic_score(&grid, &x, &y, height))
        })
        .max()
        .expect("cannot get highest scenic score")
}

fn build_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0' as u8).collect())
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = build_grid(input);
    let part1 = part1(&grid);
    let part2 = part2(&grid);

    println!("{part1}\n{part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../test-input.txt");

    #[test]
    fn test_part1() {
        let grid = build_grid(INPUT);

        assert_eq!(21, part1(&grid));
    }

    #[test]
    fn test_part2() {
        let grid = build_grid(INPUT);

        assert_eq!(16, part2(&grid));
    }
}

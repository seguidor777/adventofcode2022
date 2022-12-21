use std::collections::BTreeMap;

const START_COORD: Coord = (500, 0);

type Map = BTreeMap<Coord, Tile>;
type Coord = (usize, usize);

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Rock,
    Sand,
}

fn build_map(input: &str) -> (Map, usize) {
    let mut map = BTreeMap::new();
    let mut max_y = 0;

    input.lines().for_each(|line| {
        line.split(" -> ")
            .map(|coord| {
                let (x, y) = coord.split_once(",").expect("cannot split position");

                (
                    x.parse().expect("cannot parse x"),
                    y.parse().expect("cannot parse y"),
                )
            })
            .collect::<Vec<Coord>>()
            .windows(2)
            .for_each(|window| {
                let (x_start, y_start) = window[0];
                let (x_end, y_end) = window[1];

                (x_start.min(x_end)..=x_start.max(x_end)).for_each(|x| {
                    (y_start.min(y_end)..=y_start.max(y_end)).for_each(|y| {
                        if y > max_y {
                            max_y = y
                        }

                        map.insert((x, y), Tile::Rock);
                    })
                })
            })
    });

    (map, max_y)
}

fn simulate(
    map: &mut Map,
    stop_condition: impl Fn(Coord) -> bool,
    on_the_floor: impl Fn(usize) -> bool,
) {
    loop {
        let (mut x, mut y) = (START_COORD.0 as isize, START_COORD.1);

        loop {
            match [x, x - 1, x + 1].iter().find(|&&next_x| {
                !map.contains_key(&(next_x as usize, y + 1)) && on_the_floor(y + 1)
            }) {
                Some(&next_x) => {
                    x = next_x;
                    y += 1;
                }
                None => {
                    if map.insert((x as usize, y), Tile::Sand).is_none() {
                        break
                    }
                }
            };

            if stop_condition((x as usize, y)) {
                return;
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let (mut map, max_y) = build_map(input);

    simulate(&mut map, |(_, y)| y > max_y, |_| true);
    map.values().filter(|&&tile| tile == Tile::Sand).count()
}

fn part2(input: &str) -> usize {
    let (mut map, max_y) = build_map(input);

    simulate(&mut map, |coord| coord == START_COORD, |y| y < max_y + 2);
    map.values().filter(|&&tile| tile == Tile::Sand).count()
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
        assert_eq!(24, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(93, part2(INPUT));
    }
}

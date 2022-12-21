use std::collections::{BTreeMap, HashSet};

const SIGNAL_MULTIPLIER: isize = 4_000_000;

type Map = BTreeMap<Coord, Tile>;
type Coord = (isize, isize);

#[derive(Clone, Copy)]
enum Tile {
    Sensor(isize),
    Beacon,
}

fn manhattan_distance((x1, y1): &Coord, (x2, y2): &Coord) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn build_map(input: &str) -> Map {
    let mut map = Map::new();

    input.lines().for_each(|line| {
        let [sx, sy, bx, by]: [_; 4] = line
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter_map(|x| x.parse::<isize>().ok())
            .collect::<Vec<isize>>()
            .try_into()
            .expect("malformed line");
        map.insert(
            (sx, sy),
            Tile::Sensor(manhattan_distance(&(sx, sy), &(bx, by))),
        );
        map.insert((bx, by), Tile::Beacon);
    });

    map
}

fn part1(input: &str, row: isize) -> usize {
    let map = build_map(input);
    let mut no_beacon = HashSet::new();

    map.iter()
        .filter(|(_, tile)| matches!(tile, Tile::Sensor(..)))
        .for_each(|(&(sx, sy), tile)| {
            if let Tile::Sensor(beacon_distance) = tile {
                let row_distance = (sy - row).abs();
                let delta_distance = beacon_distance - row_distance;

                for x in (sx - delta_distance)..=(sx + delta_distance) {
                    if map.get(&(x, row)).is_none() {
                        no_beacon.insert((x, row));
                    }
                }
            }
        });

    no_beacon.len()
}

fn part2(input: &str, max_xy: isize) -> isize {
    let map = build_map(input);
    let signals: Vec<(Coord, isize)> = map
        .iter()
        .filter_map(|(coord, tile)| match tile {
            Tile::Sensor(distance) => Some((*coord, *distance)),
            _ => None,
        })
        .collect();

    for y in 0..=max_xy {
        let mut x = 0;

        'x_loop: while x <= max_xy {
            for ((sx, sy), beacon_distance) in &signals {
                let distance = manhattan_distance(&(x, y), &(*sx, *sy));

                if distance > *beacon_distance {
                    continue;
                }

                let y_distance = (sy - y).abs();
                let delta_distance = beacon_distance - y_distance;
                let max_x = sx + delta_distance;

                x = max_x + 1;

                continue 'x_loop;
            }

            // Outside all sensors coverage
            return SIGNAL_MULTIPLIER * x + y;
        }
    }

    unreachable!();
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input, 2_000_000);
    let part2 = part2(input, 4_000_000);

    println!("{part1}\n{part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../test-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(26, part1(INPUT, 10));
    }

    #[test]
    fn test_part2() {
        assert_eq!(56_000_011, part2(INPUT, 20));
    }
}

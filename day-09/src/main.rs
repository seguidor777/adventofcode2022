use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

struct Move(i32, i32, u8);

impl FromStr for Move {
    type Err = Box<dyn Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (direction, steps) = line.split_once(" ").expect("cannot parse line");
        let steps = steps.parse()?;

        Ok(match direction {
            "U" => Self(0, 1, steps),
            "L" => Self(-1, 0, steps),
            "R" => Self(1, 0, steps),
            "D" => Self(0, -1, steps),
            _ => unreachable!(),
        })
    }
}

type Pos = (i32, i32);

#[derive(Default)]
struct Rope {
    knots: Vec<Pos>,
    moves: Vec<Move>,
}

impl Rope {
    fn new(input: &str) -> Self {
        Self {
            moves: input
                .lines()
                .filter_map(|line| line.parse::<Move>().ok())
                .collect(),
            ..Default::default()
        }
    }

    fn simulate(&mut self, rope_size: usize) -> usize {
        self.knots = vec![(0, 0); rope_size];

        self.moves
            .iter()
            .fold(
                HashSet::new(),
                |mut visited: HashSet<Pos>, &Move(x, y, steps)| {
                    for _ in 0..steps {
                        self.knots[0] = (self.knots[0].0 + x, self.knots[0].1 + y);

                        for i in 1..rope_size {
                            let (head, tail) = (self.knots[i - 1], self.knots[i]);
                            let (dx, dy) = (head.0 - tail.0, head.1 - tail.1);

                            if dx.abs() > 1 || dy.abs() > 1 {
                                self.knots[i] = (tail.0 + dx.signum(), tail.1 + dy.signum())
                            }
                        }

                        // Tail of the rope is the last of the knots
                        visited.insert(self.knots[rope_size - 1]);
                    }

                    visited
                },
            )
            .len()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = Rope::new(input).simulate(2);
    let part2 = Rope::new(input).simulate(10);

    println!("{part1}\n{part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test-input.txt");
        assert_eq!(13, Rope::new(input).simulate(2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test-input-larger.txt");
        assert_eq!(36, Rope::new(input).simulate(10));
    }
}

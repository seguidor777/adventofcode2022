use std::cmp::Reverse;
use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operand: Option<u64>,
    operation: fn(u64, u64) -> u64,
    divisible_by: u64,
    monkey_true: usize,
    monkey_false: usize,
    counted: u64,
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(paragraph: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = paragraph.split("\n").collect();
        let items = lines[1][18..]
            .split(", ")
            .filter_map(|value| value.parse().ok())
            .collect();
        let operand = lines[2][25..].parse::<u64>().ok();
        let operation = if lines[2].contains('+') {
            |item, rhs| item + rhs
        } else {
            |item, rhs| item * rhs
        };
        let divisible_by: u64 = lines[3][21..].parse().expect("cannot parse divisible_by");
        let monkey_true: usize = lines[4][29..].parse().expect("cannot parse monkey_true");
        let monkey_false: usize = lines[5][30..].parse().expect("cannot parse monkey_false");

        Ok(Self {
            items,
            operand,
            operation,
            divisible_by,
            monkey_true,
            monkey_false,
            counted: 0,
        })
    }
}

fn play_keep_away(mut monkeys: Vec<Monkey>, rounds: u32, reducer: impl Fn(u64) -> u64) -> u64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            monkeys[i].counted += monkeys[i].items.len() as u64;

            while let Some(item) = monkeys[i].items.pop_front() {
                let rhs = if let Some(o) = monkeys[i].operand { o } else { item };
                let worry_level = reducer((monkeys[i].operation)(item, rhs));
                let destination = if worry_level % monkeys[i].divisible_by == 0 {
                    monkeys[i].monkey_true
                } else {
                    monkeys[i].monkey_false
                };

                monkeys[destination].items.push_back(worry_level);
            }
        }
    }

    monkeys.sort_by_key(|monkey| Reverse(monkey.counted));
    monkeys
        .iter()
        .map(|monkey| monkey.counted)
        .take(2)
        .product()
}

fn main() {
    let monkeys: Vec<Monkey> = include_str!("../input.txt")
        .split("\n\n")
        .filter_map(|paragraph| paragraph.parse::<Monkey>().ok())
        .collect();
    let part1 = play_keep_away(monkeys.clone(), 20, |item| item / 3);
    let modulus: u64 = monkeys
        .iter()
        .map(|monkey| monkey.divisible_by)
        .product();
    let part2 = play_keep_away(monkeys, 10_000, |item| item % modulus);

    println!("{part1}\n{part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../test-input.txt");

    #[test]
    fn test_part1() {
        let monkeys: Vec<Monkey> = INPUT
            .split("\n\n")
            .filter_map(|paragraph| paragraph.parse::<Monkey>().ok())
            .collect();
        assert_eq!(10_605, play_keep_away(monkeys, 20, |item| item / 3));
    }

    #[test]
    fn test_part2() {
        let monkeys: Vec<Monkey> = INPUT
            .split("\n\n")
            .filter_map(|paragraph| paragraph.parse::<Monkey>().ok())
            .collect();
        let modulus: u64 = monkeys
            .iter()
            .map(|monkey| monkey.divisible_by)
            .product();
        assert_eq!(2_713_310_158, play_keep_away(monkeys, 10_000, |item| item % modulus));
    }
}

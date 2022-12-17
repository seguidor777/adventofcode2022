use serde_json::Value;
use std::cmp::Ordering;

fn compare(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Array(a_array), Value::Array(b_array)) => {
            for i in 0..a_array.len().max(b_array.len()) {
                match (a_array.get(i), b_array.get(i)) {
                    (None, _) => return Ordering::Less,
                    (_, None) => return Ordering::Greater,
                    (Some(a_value), Some(b_value)) => match compare(a_value, b_value) {
                        Ordering::Equal => {}
                        ordering => return ordering,
                    },
                }
            }

            Ordering::Equal
        }
        (Value::Number(a_number), Value::Number(b_number)) => {
            a_number.as_u64().unwrap().cmp(&b_number.as_u64().unwrap())
        }
        (Value::Number(_), Value::Array(_)) => compare(&Value::Array(vec![left.clone()]), right),
        (Value::Array(_), Value::Number(_)) => compare(left, &Value::Array(vec![right.clone()])),
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .fold((1, 0), |(i, acc), pair| {
            let [left, right]: [_; 2] = pair
                .split("\n")
                .take(2)
                .map(|pair| serde_json::from_str::<Value>(pair).expect("cannot deserialize JSON"))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            if compare(&left, &right) == Ordering::Less {
                return (i + 1, acc + i);
            }

            (i + 1, acc)
        })
        .1
}

fn part2(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .split("\n")
        .filter(|packet| !packet.is_empty())
        .map(|packet| serde_json::from_str::<Value>(packet).expect("cannot deserialize JSON"))
        .collect();
    let dividers: Vec<_> = ["[[2]]", "[[6]]"]
        .iter()
        .map(|packet| serde_json::from_str::<Value>(packet).unwrap())
        .collect();

    packets.extend(dividers.clone());
    packets.sort_by(compare);

    (1..=packets.len())
        .filter(|&i| dividers.contains(&packets[i - 1]))
        .product()
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
        assert_eq!(13, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(140, part2(INPUT));
    }
}

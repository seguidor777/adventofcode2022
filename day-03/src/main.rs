fn find_common(a: &[u8], b: &[u8]) -> u8 {
    a.iter()
        .copied()
        .find(|item| b.contains(item))
        .expect("no common item")
}

fn find_commons(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().copied().filter(|item| b.contains(item)).collect()
}

fn get_priority(ch: u8) -> u32 {
    (match ch {
        b'a'..=b'z' => ch - b'a' + 1,
        b'A'..=b'Z' => ch - b'A' + 27,
        _ => unreachable!(),
    }) as u32
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|l| l.as_bytes())
        .collect::<Vec<_>>();
    let priorities_sum: u32 = input
        .iter()
        .map(|l| {
            let (a, b) = l.split_at(l.len() >> 1);
            get_priority(find_common(a, b))
        })
        .sum();

    let badge_priorities_sum: u32 = input
        .chunks(3)
        .map(|chunks| {
            let [a, b, c]: [_; 3] = chunks.try_into().unwrap();
            get_priority(find_common(&find_commons(a, b), c))
        })
        .sum();

    println!("{priorities_sum}\n{badge_priorities_sum}");
}

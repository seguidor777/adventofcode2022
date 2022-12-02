use std::time::Instant;

fn eval_part1(player1: &u8, player2: &u8) -> u32 {
    match (player1, player2) {
        (b'A', b'X') => 4,
        (b'A', b'Y') => 8,
        (b'A', b'Z') => 3,
        (b'B', b'X') => 1,
        (b'B', b'Y') => 5,
        (b'B', b'Z') => 9,
        (b'C', b'X') => 7,
        (b'C', b'Y') => 2,
        (b'C', b'Z') => 6,
        _ => 0,
    }
}

fn eval_part2(player1: &u8, player2: &u8) -> u32 {
    match (player1, player2) {
        (b'A', b'X') => 3,
        (b'A', b'Y') => 4,
        (b'A', b'Z') => 8,
        (b'B', b'X') => 1,
        (b'B', b'Y') => 5,
        (b'B', b'Z') => 9,
        (b'C', b'X') => 2,
        (b'C', b'Y') => 6,
        (b'C', b'Z') => 7,
        _ => 0,
    }
}

fn main() {
    let matches: Vec<(u8, u8)> = include_str!("../input.txt")
        .lines()
        .map(|l| { let bytes = l.as_bytes(); (bytes[0], bytes[2]) })
        .collect();
    let score1: u32 = matches.iter().map(|(p1, p2)| eval_part1(p1, p2)).sum();
    let score2: u32 = matches.iter().map(|(p1, p2)| eval_part2(p1, p2)).sum();

    println!("{score1}\n{score2}");
}

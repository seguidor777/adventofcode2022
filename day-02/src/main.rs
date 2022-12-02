fn eval_part1(player1: &char, player2: &char) -> u32 {
    match (player1, player2) {
        ('A', 'X') => 4,
        ('A', 'Y') => 8,
        ('A', 'Z') => 3,
        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,
        ('C', 'X') => 7,
        ('C', 'Y') => 2,
        ('C', 'Z') => 6,
        _ => 0,
    }
}

fn eval_part2(player1: &char, player2: &char) -> u32 {
    match (player1, player2) {
        ('A', 'X') => 3,
        ('A', 'Y') => 4,
        ('A', 'Z') => 8,
        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,
        ('C', 'X') => 2,
        ('C', 'Y') => 6,
        ('C', 'Z') => 7,
        _ => 0,
    }
}

fn main() {
    let matches: Vec<(char, char)> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let bytes = l.as_bytes();

            (bytes[0] as char, bytes[2] as char)
        })
        .collect();
    let score1: u32 = matches.iter().map(|(p1, p2)| eval_part1(p1, p2)).sum();
    let score2: u32 = matches.iter().map(|(p1, p2)| eval_part2(p1, p2)).sum();

    println!("{score1}\n{score2}");
}

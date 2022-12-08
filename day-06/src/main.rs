fn find_marker(char_bytes: &[u8], window_size: usize) -> Option<usize> {
    char_bytes
        .windows(window_size)
        .enumerate()
        .find_map(|(i, window)| {
            let mut checker = 0;

            for ch in window {
                let val = ch - b'a';
                if (checker & 1 << val) > 0 {
                    return None;
                }
                checker |= 1 << val
            }

            Some(i + window_size)
        })
}

fn main() {
    let char_bytes = include_str!("../input.txt").as_bytes();
    let part1 = find_marker(char_bytes, 4).expect("market not found");
    let part2 = find_marker(char_bytes, 14).expect("market not found");

    println!("{part1}\n{part2}");
}

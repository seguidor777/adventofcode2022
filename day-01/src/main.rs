fn main() {
    let mut totals: Vec<u32> = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect();
    totals.sort_by(|a, b| b.cmp(a));
    let max_total = totals[0];
    let n_totals_sum = totals[0..3].iter().sum::<u32>();

    println!("{max_total}\n{n_totals_sum}");
}

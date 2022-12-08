use std::ops::RangeInclusive;

fn range_contains(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

fn range_overlaps(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}

fn main() {
    let ranges: Vec<_> = include_str!("../input.txt")
        .lines()
        .filter_map(|line| {
            let (a_range, b_range) = line.split_once(',')?;
            [a_range, b_range]
                .map(|range| {
                    let [start, end]: [_; 2] = range
                        .splitn(2, '-')
                        .filter_map(|x| x.parse::<u32>().ok())
                        .collect::<Vec<u32>>()
                        .try_into()
                        .unwrap();
                    start..=end
                })
                .into()
        })
        .collect();
    let part1 = ranges
        .iter()
        .filter(|[a_range, b_range]| {
            range_contains(a_range, b_range) || range_contains(b_range, a_range)
        })
        .count();
    let part2 = ranges
        .iter()
        .filter(|[a_range, b_range]| range_overlaps(a_range, b_range))
        .count();

    println!("{part1}\n{part2}");
}

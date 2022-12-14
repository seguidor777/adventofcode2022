fn part1(lines: &[&str]) -> i32 {
    let increase_signal = |cycle: &i32, x: &i32, signal: &mut i32| {
        if (cycle - 20) % 40 == 0 {
            *signal += cycle * x
        }
    };

    lines
        .iter()
        .fold((0, 1, 0), |(mut cycle, mut x, mut signal), &line| {
            cycle += 1;
            increase_signal(&cycle, &x, &mut signal);

            if line.len() > 4 {
                cycle += 1;
                increase_signal(&cycle, &x, &mut signal);

                let v = line[5..].parse::<i32>().expect("cannot parse v");
                x += v;
            }

            (cycle, x, signal)
        })
        .2
}

fn draw_pixel(cycle: &i32, x: &i32, screen: &mut String) {
    if *cycle > 0 && cycle % 40 == 0 {
        screen.push('\n')
    }

    let pixel = if (x - cycle % 40).abs() < 2 { '#' } else { '.' };

    screen.push(pixel);
}

fn part2(lines: &[&str]) -> String {
    lines
        .iter()
        .fold(
            (0, 1, String::new()),
            |(mut cycle, mut x, mut screen), &line| {
                draw_pixel(&cycle, &x, &mut screen);
                cycle += 1;

                if line.len() > 4 {
                    draw_pixel(&cycle, &x, &mut screen);
                    cycle += 1;

                    let v = line[5..].parse::<i32>().expect("cannot parse v");
                    x += v;
                }

                (cycle, x, screen)
            },
        )
        .2
}

fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);

    println!("{part1}\n{part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../test-input.txt");

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = INPUT.lines().collect();
        assert_eq!(13140, part1(&lines));
    }

    #[test]
    fn test_part2() {
        let lines: Vec<&str> = INPUT.lines().collect();
        assert_eq!(include_str!("../test-image.txt"), &part2(&lines));
    }
}

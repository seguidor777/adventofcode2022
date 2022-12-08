fn get_message(stacks: &Vec<Vec<char>>) -> String {
    stacks.iter().filter_map(|stack| stack.last()).collect()
}

fn move_with_9000(mut stacks: Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    for &(quantity, from, to) in moves {
        for _ in 0..quantity {
            let item = stacks[from - 1].pop().expect("no items left");
            stacks[to - 1].push(item);
        }
    }

    get_message(&stacks)
}

fn move_with_9001(mut stacks: Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    for &(quantity, from, to) in moves {
        let new_length = stacks[from - 1].len() - quantity;
        let mut items = stacks[from - 1].drain(new_length..).collect::<Vec<char>>();
        stacks[to - 1].append(&mut items);
    }

    get_message(&stacks)
}

fn main() {
    let (stacks_input, moves_input) = include_str!("../input.txt")
        .split_once("\n\n")
        .expect("cannot split input");
    let mut stacks = vec![vec![]; 9];

    stacks_input.lines().rev().for_each(|line| {
        let line = line.as_bytes();
        stacks.iter_mut().enumerate().for_each(|(i, stack)| {
            let ch = line[i * 4 + 1];
            if ch.is_ascii_alphabetic() {
                stack.push(ch as char);
            }
        })
    });

    let moves = moves_input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .filter_map(|number| number.parse::<usize>().ok())
                .collect::<Vec<_>>();

            (numbers[0], numbers[1], numbers[2])
        })
        .collect::<Vec<_>>();

    let part1 = move_with_9000(stacks.clone(), &moves);
    let part2 = move_with_9001(stacks, &moves);

    println!("{part1}\n{part2}");
}

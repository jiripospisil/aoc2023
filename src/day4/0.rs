use std::{collections::HashSet, io};

fn main() {
    let total = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, curr| {
            let mut pieces = curr.split(':').last().unwrap().split('|');

            let winning = pieces
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            let have = pieces
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            acc + winning
                .intersection(&have)
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        });

    println!("{total}");
}

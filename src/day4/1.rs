use std::{
    collections::{HashMap, HashSet},
    io,
};

fn main() {
    let mut cards = HashMap::<usize, u32>::new();

    for (idx, line) in io::stdin().lines().map(|line| line.unwrap()).enumerate() {
        let idx = idx + 1;
        let count = *cards.entry(idx).and_modify(|e| *e += 1).or_insert(1);

        let mut pieces = line.split(':').last().unwrap().split('|');

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

        for (idxx, _) in winning.intersection(&have).enumerate() {
            cards
                .entry(idx + idxx + 1)
                .and_modify(|e| *e += count)
                .or_insert(count);
        }
    }

    println!("{}", cards.values().sum::<u32>());
}

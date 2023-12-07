use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, io};

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn main() {
    let total = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut pieces = line.split_ascii_whitespace();
            let hand = pieces.next().unwrap().to_owned();
            let bid = pieces.next().unwrap().parse::<u32>().unwrap();
            let rank = get_rank_with_jokers(&hand);

            (hand, bid, rank)
        })
        .sorted_unstable_by(|a, b| {
            let mut res = a.2.cmp(&b.2);
            let mut iter = a.0.chars().zip(b.0.chars());

            while res == Ordering::Equal {
                let next = iter.next().unwrap();
                let idx_a = CARDS.iter().position(|c| *c == next.0).unwrap();
                let idx_b = CARDS.iter().position(|c| *c == next.1).unwrap();

                res = idx_a.cmp(&idx_b).reverse();
            }

            res
        })
        .enumerate()
        .fold(0, |acc, (idx, curr)| acc + (idx + 1) as u32 * curr.1);

    println!("{total}");
}

fn get_rank_with_jokers(hand: &str) -> u8 {
    if !hand.contains('J') {
        return get_rank(hand);
    }

    let indices = hand.chars().positions(|c| c == 'J').collect_vec();

    let products = (0..indices.len())
        .map(|_| CARDS)
        .multi_cartesian_product()
        .collect_vec();

    let mut candidates = Vec::new();

    for product in &products {
        let mut candidate = hand.to_owned();
        for (idx, val) in product.iter().enumerate() {
            candidate.replace_range(indices[idx]..indices[idx] + 1, &val.to_string());
        }
        candidates.push(candidate);
    }

    candidates.iter().map(|c| get_rank(c)).max().unwrap()
}

fn get_rank(hand: &str) -> u8 {
    let types = hand
        .chars()
        .sorted_unstable_by_key(|k| CARDS.iter().position(|c| c == k).unwrap())
        .fold(HashMap::new(), |mut acc, curr| {
            acc.entry(curr).and_modify(|e| *e += 1).or_insert(1);
            acc
        });

    let values = types.values().collect_vec();

    if values.contains(&&5) {
        return 7;
    }

    if values.contains(&&4) {
        return 6;
    }

    if values.contains(&&3) && values.contains(&&2) {
        return 5;
    }

    if values.contains(&&3) {
        return 4;
    }

    if values.contains(&&2) && values.len() == 3 {
        return 3;
    }

    if values.contains(&&2) && values.len() == 4 {
        return 2;
    }

    1
}

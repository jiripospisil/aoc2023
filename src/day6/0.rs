use itertools::Itertools;
use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"(\d+)").unwrap();

    let numbers = re
        .captures_iter(&input)
        .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
        .collect_vec();

    let total = numbers
        .iter()
        .take(4)
        .enumerate()
        .map(|(idx, curr)| {
            let record = numbers[idx + 4];

            (1..=*curr)
                .filter(|n| {
                    let time = curr - n;
                    let distance = n * time;
                    distance > record
                })
                .count() as u64
        })
        .product::<u64>();

    println!("{total}");
}

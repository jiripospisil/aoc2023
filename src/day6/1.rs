use itertools::Itertools;
use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"(\d+)").unwrap();

    let numbers = re
        .captures_iter(&input)
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|cap| cap.get(1).unwrap().as_str())
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .collect_vec();

    let time = numbers[0];
    let record = numbers[1];
    let total = (1..=time)
        .filter(|n| {
            let time = time - n;
            let distance = n * time;
            distance > record
        })
        .count();

    println!("{total}");
}

use itertools::Itertools;
use std::io;

fn main() {
    let total = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
        .fold(0, |acc, (idx, curr)| {
            for set in curr.split(&[':', ';']).skip(1) {
                let pairs = set.split(',').flat_map(|s| s.split_whitespace());

                for (count, color) in pairs.tuples() {
                    let count: u32 = count.parse().expect("valid number");

                    match color {
                        "red" if count > 12 => return acc,
                        "green" if count > 13 => return acc,
                        "blue" if count > 14 => return acc,
                        _ => {}
                    };
                }
            }

            acc + idx + 1
        });

    println!("{total}");
}

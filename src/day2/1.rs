use itertools::Itertools;
use std::io;

fn main() {
    let total = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, curr| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for set in curr.split(&[':', ';']).skip(1) {
                let pairs = set.split(',').flat_map(|s| s.split_whitespace());

                for (count, color) in pairs.tuples() {
                    let count: u32 = count.parse().expect("valid number");

                    match color {
                        "red" => red = red.max(count),
                        "green" => green = green.max(count),
                        "blue" => blue = blue.max(count),
                        _ => {}
                    };
                }
            }

            acc + (red * green * blue)
        });

    println!("{total}");
}

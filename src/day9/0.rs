use itertools::Itertools;
use std::io;

fn main() {
    let total = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, curr| {
            let numbers = curr
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect_vec();

            let mut rows = Vec::new();
            rows.push(numbers);

            loop {
                if rows.last().unwrap().iter().all(|n| *n == 0) {
                    break;
                }

                let row = rows
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| *b - *a)
                    .collect_vec();

                rows.push(row);
            }

            acc + rows.iter().map(|row| row.last().unwrap()).sum::<i32>()
        });

    println!("{total}");
}

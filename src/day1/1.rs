use std::io;

use aho_corasick::AhoCorasick;

fn main() {
    let patterns = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();

    let total = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, curr| {
            let mut iter = ac.find_overlapping_iter(&curr).peekable();

            let left = iter.peek().expect("left number").pattern().as_usize();
            let left = if left < 9 { left + 1 } else { left - 8 };

            let right = iter.last().expect("right number").pattern().as_usize();
            let right = if right < 9 { right + 1 } else { right - 8 };

            acc + left * 10 + right
        });

    println!("{total}");
}

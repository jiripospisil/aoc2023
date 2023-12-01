use std::io;

fn main() {
    let total = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .fold(0u32, |acc, curr| {
            let left = curr
                .chars()
                .find(|c| c.is_ascii_digit())
                .expect("left number")
                .to_digit(10)
                .expect("number is number");

            let right = curr
                .chars()
                .rfind(|c| c.is_ascii_digit())
                .expect("right number")
                .to_digit(10)
                .expect("number is number");

            acc + left * 10 + right
        });

    println!("{total}");
}

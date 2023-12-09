use std::{
    collections::HashMap,
    io::{self, Read},
};

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let instructions = lines.next().unwrap();

    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]+)\)").unwrap();

    let network = lines.skip(1).fold(HashMap::new(), |mut network, curr| {
        let caps = re.captures(curr).unwrap();

        network.insert(
            caps.get(1).unwrap().as_str(),
            (caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str()),
        );

        network
    });

    let mut current = network["AAA"];
    let mut count = 0;

    for ins in instructions.chars().cycle() {
        if ins == 'L' {
            if current.0 == "ZZZ" {
                count += 1;
                break;
            }
            current = network[current.0];
        } else {
            if current.1 == "ZZZ" {
                count += 1;
                break;
            }
            current = network[current.1];
        }

        count += 1;
    }

    println!("{count}");
}

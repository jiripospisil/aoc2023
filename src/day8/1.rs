use std::{
    collections::HashMap,
    io::{self, Read},
};

use itertools::Itertools;
use num::BigInt;
use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let instructions = lines.next().unwrap();

    let re = Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap();

    let network = lines.skip(1).fold(HashMap::new(), |mut network, curr| {
        let caps = re.captures(curr).unwrap();

        network.insert(
            caps.get(1).unwrap().as_str(),
            (caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str()),
        );

        network
    });

    let nodes = network
        .iter()
        .filter_map(|k| if k.0.ends_with('A') { Some(k.1) } else { None })
        .collect_vec();

    let total = nodes
        .into_iter()
        .map(|node| {
            let mut count = 0;
            let mut current = node;

            for ins in instructions.chars().cycle() {
                count += 1;
                if ins == 'L' {
                    if current.0.ends_with('Z') {
                        return count;
                    }
                    current = &network[current.0];
                } else {
                    if current.1.ends_with('Z') {
                        return count;
                    }
                    current = &network[current.1];
                }
            }
            count
        })
        .map(BigInt::from)
        .reduce(num::integer::lcm)
        .unwrap();

    println!("{total}");
}

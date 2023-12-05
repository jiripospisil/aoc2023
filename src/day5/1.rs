use itertools::Itertools;
use std::io::{self, Read};

use regex::Regex;

macro_rules! parse_cap {
    ($caps:ident, $idx:literal) => {
        $caps.get($idx).unwrap().as_str().parse::<u64>().unwrap()
    };
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut seed_ranges = Vec::new();
    let mut maps = Vec::new();

    let re_seeds = Regex::new(r"(\d+)+").unwrap();
    let re_maps = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    for group in input.split("\n\n") {
        if group.starts_with("seeds:") {
            seed_ranges.extend(
                re_seeds
                    .captures_iter(group)
                    .map(|cap| parse_cap!(cap, 0))
                    .tuples()
                    .map(|(start, length)| start..start + length),
            );
        } else {
            let mut map = Vec::new();
            for caps in re_maps.captures_iter(group) {
                let dest = parse_cap!(caps, 1);
                let src = parse_cap!(caps, 2);
                let length = parse_cap!(caps, 3);

                map.push((dest, src..src + length));
            }
            maps.push(map);
        }
    }

    let mut min = u64::MAX;
    for seed_range in seed_ranges {
        for seed in seed_range {
            let mut val = seed;

            for map in &maps {
                for entry in map {
                    if entry.1.contains(&val) {
                        val = entry.0 + val - entry.1.start;
                        break;
                    }
                }
            }

            if val < min {
                min = val;
            }
        }
    }

    println!("{min}");
}

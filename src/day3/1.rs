use std::io;

#[derive(Clone, Copy, Debug)]
enum Item {
    Period,
    Symbol(char),
    Number(u32),
}

fn main() {
    let mut grid: Vec<Vec<Item>> = vec![];

    for (idl, line) in io::stdin().lines().map(|line| line.unwrap()).enumerate() {
        grid.push(vec![]);

        let mut digit = 0u32;
        let mut digit_start = 0;

        for (idr, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if digit == 0 {
                    digit_start = idr;
                }
                digit = digit * 10 + c.to_digit(10).unwrap();
                continue;
            }

            if digit != 0 {
                for _ in digit_start..idr {
                    grid[idl].push(Item::Number(digit));
                }
                digit = 0;
            }

            grid[idl].push(if c == '.' {
                Item::Period
            } else {
                Item::Symbol(c)
            });
        }

        if digit != 0 {
            for _ in digit_start..line.len() {
                grid[idl].push(Item::Number(digit));
            }
        }
    }

    let mut total = 0;

    for (idy, y) in grid.iter().enumerate() {
        let idy = idy as i32;
        for (idx, x) in y.iter().enumerate() {
            let idx = idx as i32;
            let mut subtotal = 1;
            let mut count = 0;
            match x {
                Item::Symbol(c) if *c == '*' => {
                    checked_mul(&grid, &mut subtotal, &mut count, idy, idx - 1);
                    checked_mul(&grid, &mut subtotal, &mut count, idy, idx + 1);

                    if !checked_mul(&grid, &mut subtotal, &mut count, idy - 1, idx) {
                        checked_mul(&grid, &mut subtotal, &mut count, idy - 1, idx - 1);
                        checked_mul(&grid, &mut subtotal, &mut count, idy - 1, idx + 1);
                    }

                    if !checked_mul(&grid, &mut subtotal, &mut count, idy + 1, idx) {
                        checked_mul(&grid, &mut subtotal, &mut count, idy + 1, idx - 1);
                        checked_mul(&grid, &mut subtotal, &mut count, idy + 1, idx + 1);
                    }

                    if count == 2 {
                        total += subtotal;
                    }
                }
                _ => (),
            }
        }
    }

    println!("{total}");
}

fn checked_mul(grid: &Vec<Vec<Item>>, subtotal: &mut u32, count: &mut u32, y: i32, x: i32) -> bool {
    if y >= 0 && (y as usize) < grid.len() {
        let row = &grid[y as usize];

        if x >= 0 && (x as usize) < row.len() {
            if let Item::Number(n) = row[x as usize] {
                *subtotal *= n;
                *count += 1;
                return true;
            }
        }
    }

    false
}

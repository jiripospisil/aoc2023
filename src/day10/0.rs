use core::panic;
use std::{collections::HashSet, io};

struct Elem {
    visited: HashSet<(i32, i32)>,
    x: i32,
    y: i32,
}

fn main() {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut start = (0, 0);

    for (idl, line) in io::stdin().lines().map(|line| line.unwrap()).enumerate() {
        grid.push(vec![]);

        for (idr, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (idr as i32, idl as i32);
            }
            grid[idl].push(c);
        }
    }

    let mut paths = Vec::new();

    add_around_start(&grid, &mut paths, &['-', 'L', 'F'], &start, (-1, 0));
    add_around_start(&grid, &mut paths, &['-', 'J', '7'], &start, (1, 0));
    add_around_start(&grid, &mut paths, &['|', 'F', '7'], &start, (0, -1));
    add_around_start(&grid, &mut paths, &['|', 'L', 'J'], &start, (0, 1));

    let mut max = 0;

    while let Some(curr) = paths.pop() {
        let visited = &curr.visited;
        let x = curr.x;
        let y = curr.y;

        if grid[y as usize][x as usize] == 'S' {
            if visited.len() > max {
                max = visited.len();
            }
            continue;
        }

        add_paths(&grid, &mut paths, &curr);
    }

    println!("{}", (max + 1) / 2);
}

fn add_paths(grid: &[Vec<char>], paths: &mut Vec<Elem>, curr: &Elem) {
    let x = curr.x;
    let y = curr.y;
    let tile = grid[y as usize][x as usize];

    let directions = &match tile {
        'J' => [(x - 1, y), (x, y - 1)],
        'F' => [(x + 1, y), (x, y + 1)],
        '|' => [(x, y - 1), (x, y + 1)],
        '-' => [(x - 1, y), (x + 1, y)],
        '7' => [(x - 1, y), (x, y + 1)],
        'L' => [(x, y - 1), (x + 1, y)],
        _ => panic!("{}", &tile),
    };

    for direction in directions {
        let x = direction.0;
        let y = direction.1;

        if curr.visited.contains(&(x, y)) {
            continue;
        }

        if x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid.len() as i32 {
            let mut set = curr.visited.clone();
            set.insert((x, y));
            paths.push(Elem { visited: set, x, y })
        }
    }
}

fn add_around_start(
    grid: &[Vec<char>],
    paths: &mut Vec<Elem>,
    tiles: &[char],
    start: &(i32, i32),
    m: (i32, i32),
) {
    let x = start.0 + m.0;
    let y = start.1 + m.1;

    if x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid.len() as i32 {
        for tile in tiles {
            if grid[y as usize][x as usize] == *tile {
                paths.push(Elem {
                    visited: HashSet::new(),
                    x,
                    y,
                })
            }
        }
    }
}

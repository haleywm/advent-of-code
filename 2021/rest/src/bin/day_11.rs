use std::cmp::min;
use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input/day_11.txt").expect("Invalid Filename");
    let mut grid: Vec<Vec<u32>> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|num| num.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut grid_two = grid.clone();

    assert_eq!(grid.len(), 10);
    for line in grid.iter() {
        assert_eq!(line.len(), 10);
    }

    // Part 1
    let mut total = 0;
    for _ in 0..100 {
        total += step(&mut grid);
    }

    println!("{}", total);

    // Part 2
    let mut steps = 1;
    while step(&mut grid_two) != 100 {
        steps += 1;
    }

    println!("{}", steps);
}

fn step(grid: &mut Vec<Vec<u32>>) -> usize {
    let mut to_flash: Vec<(usize, usize)> = Vec::new();
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut total = 0;

    // Incrementing all items, counting the ones that are about to flash
    for x in 0..10 {
        for y in 0..10 {
            if grid[x][y] == 9 {
                to_flash.push((x, y));
                flashed.insert((x, y));
            }
            grid[x][y] += 1;
        }
    }
    // Flashing until they're all done
    while let Some((x, y)) = to_flash.pop() {
        total += 1;
        grid[x][y] = 0;
        for mod_x in x.saturating_sub(1)..=min(x + 1, 9) {
            for mod_y in y.saturating_sub(1)..=min(y + 1, 9) {
                if (mod_x != x || mod_y != y) && !flashed.contains(&(mod_x, mod_y)) {
                    if grid[mod_x][mod_y] == 9 {
                        to_flash.push((mod_x, mod_y));
                        flashed.insert((mod_x, mod_y));
                    }
                    grid[mod_x][mod_y] += 1;
                }
            }
        }
    }

    total
}

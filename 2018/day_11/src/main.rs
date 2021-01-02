use std::fs;
use std::cmp::min;


fn main() {
    const GRID_MIN: usize = 1;
    const GRID_MAX: usize = 300;

    let sn: i64 = fs::read_to_string("input.txt").unwrap().parse().unwrap();
    
    let mut power = vec![vec![0; 300]; 300];
    for x in 0..300 {
        for y in 0..300 {
            let rack_val = (x + 1 + 10) as i64;
            let y_coord = (y + 1) as i64;
            power[x][y] = ((((rack_val * y_coord + sn) * rack_val) / 100) % 10) - 5;
        }
    }

    let mut max = ((0, 0), 0);
    for x in GRID_MIN..=GRID_MAX-3 {
        for y in GRID_MIN..=GRID_MAX-3 {
            // Then summing the 3x3 grid
            let mut total = 0;
            for x in x..x+3 {
                for y in y..y+3 {
                    total += power[x - 1][y - 1];
                }
            }
            if total > max.1 {
                max = ((x, y), total);
            }
        }
    }

    println!("{}, {}: {}", max.0.0, max.0.1, max.1);

    // Again but this time with every possible square size
    let mut max = ((0, 0, 0), 0);
    for x in GRID_MIN..=GRID_MAX {
        for y in GRID_MIN..=GRID_MAX {
            let mut total = 0;
            for square in 1..=min(GRID_MAX - x + 1, GRID_MAX - y + 1) {
                // Increasing total by the last row and column, as they are what has been added
                // Sub 2, 1 to account for zero indexed array, one to account for the square including the original x val
                let read_x = x + square - 2;
                for read_y in y-1..=y+square-2 {
                    total += power[read_x][read_y];
                }
                // Then the same for the bottom row
                let read_y = y + square - 2;
                // Taking 3 so I don't add the bottom right corner twice
                if square > 1 {
                    for read_x in x-1..=x+square-3 {
                        total += power[read_x][read_y];
                    }
                }
                if total > max.1 {
                    max = ((x, y, square), total);
                }
            }
        }
    }
    

    println!("{}, {}, {}: {}", max.0.0, max.0.1, max.0.2, max.1);
}

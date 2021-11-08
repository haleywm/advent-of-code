use std::env;
use std::cmp;

fn main() {
    let mut args = env::args();
    let size: usize = args.nth(1).and_then(|x| x.parse().ok()).unwrap_or(100);
    let steps: usize = args.next().and_then(|x| x.parse().ok()).unwrap_or(100);

    let mut grids: Vec<Vec<Vec<bool>>> = Vec::with_capacity(2);
    grids.push(vec![vec![false; size]; size]);
    grids.push(grids[0].clone());

    let init_grid = &mut grids[0];
    // Reading input
    for (x, line) in advent::line_iter("input/day_18.txt")
        .expect("Unable to read input file")
        .map(|x| x.unwrap())
        .enumerate() {
            let line = line.into_bytes();
            assert!(x < init_grid.len());
            assert_eq!(line.len(), init_grid[x].len());
            for y in 0..line.len() {
                init_grid[x][y] = line[y] == b'#';
            }
        }
    let orig_grid = init_grid.clone();
    
    let mut cur = 0;
    for _ in 0..steps {
        let next = 1 - cur;
        for x in 0..size {
            for y in 0..size {
                let mut neighbours = 0;
                for rel_x in x.saturating_sub(1)..=cmp::min(x + 1, size - 1) {
                    for rel_y in y.saturating_sub(1)..=cmp::min(y + 1, size - 1) {
                        if rel_x != x || rel_y != y {
                            if grids[cur][rel_x][rel_y] {
                                neighbours += 1;
                            }
                        }
                    }
                }
                grids[next][x][y] = (grids[cur][x][y] && (neighbours == 2 || neighbours == 3))
                    || ((!grids[cur][x][y]) && neighbours == 3);
            }
        }
        cur = next;
    }
    // Done! Now just get the total
    let total = grids[cur]
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x)
        .count();
    
    println!("{}", total);

    // Now for variant
    cur = 0;
    grids[0] = orig_grid;
    grids[0][0][0] = true;
    grids[0][0][size - 1] = true;
    grids[0][size - 1][0] = true;
    grids[0][size - 1][size - 1] = true;

    for _ in 0..steps {
        let next = 1 - cur;
        for x in 0..size {
            for y in 0..size {
                let mut neighbours = 0;
                for rel_x in x.saturating_sub(1)..=cmp::min(x + 1, size - 1) {
                    for rel_y in y.saturating_sub(1)..=cmp::min(y + 1, size - 1) {
                        if rel_x != x || rel_y != y {
                            if grids[cur][rel_x][rel_y] {
                                neighbours += 1;
                            }
                        }
                    }
                }
                grids[next][x][y] = (grids[cur][x][y] && (neighbours == 2 || neighbours == 3))
                    || ((!grids[cur][x][y]) && neighbours == 3);
            }
        }

        grids[next][0][0] = true;
        grids[next][0][size - 1] = true;
        grids[next][size - 1][0] = true;
        grids[next][size - 1][size - 1] = true;

        cur = next;
    }
    // Done! Now just get the total
    let total = grids[cur]
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x)
        .count();
    
    println!("{}", total);
}

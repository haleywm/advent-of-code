use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input/day_15.txt").expect("Invalid Filename");

    let grid: Vec<Vec<u32>> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|num| num.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    
    // Part 1
    println!("{}", cheapest_path(&grid));

    // Now generating a much larger grid
    let grid_size = grid.len();
    let big_grid_size = grid_size * 5;
    let mut big_grid: Vec<Vec<u32>> = vec![Vec::with_capacity(big_grid_size); big_grid_size];
    
    for x in 0..big_grid_size {
        for y in 0..big_grid_size {
            let orig = grid[x % grid_size][y % grid_size] - 1;
            let bonus = (x / grid_size + y / grid_size) as u32;
            big_grid[x].push(((orig + bonus) % 9) + 1);
        }
    }
    println!("{}", cheapest_path(&big_grid));
}

fn cheapest_path(grid: &Vec<Vec<u32>>) -> u32 {
    let grid_size = grid.len();
    
    // Implementing dijkstras algorithm
    // Creating a map containing all unvisited nodes
    let mut unvisited: HashMap<(usize, usize), Option<u32>> = HashMap::with_capacity(grid_size * grid_size);
    // And a map for all visited nodes
    let mut visited: HashMap<(usize, usize), u32> = HashMap::with_capacity(grid_size * grid_size);

    // Inserting all nodes into unvisited
    unvisited.insert((0, 0), Some(0));
    for x in 0..grid_size {
        for y in 0..grid_size {
            if x == 0 && y == 0 {
                continue;
            }
            unvisited.insert((x, y), None);
        }
    }

    let mut cur: (usize, usize);
    let goal = (grid_size - 1, grid_size - 1);
    while !visited.contains_key(&goal) {
        // Now selecting the next cur
        cur = *unvisited
            .iter()
            .filter(|(_, val)| val.is_some())
            .min_by_key(|(_, val)| val.unwrap())
            //.expect("Ran out of nodes to visit")
            .unwrap_or_else(|| panic!("Panicked with {} visited, {} unvisited: {:?}", visited.len(), unvisited.len(), visited))
            .0;
        let cur_cost = unvisited.get(&cur).unwrap().unwrap();
        for neighbour in get_unvisited_neighbours(cur, &visited, grid_size - 1) {
            let comp = unvisited.get_mut(&neighbour).unwrap();
            let n_cost = grid[neighbour.0][neighbour.1];
            // If the current value is None, or if it's greater than the current cost plus the travel cost, update
            if comp.map_or(true, |n_val| n_val > cur_cost + n_cost) {
                *comp = Some(cur_cost + n_cost);
            }
        }
        // Now removing cur from unvisited as it's been visited
        unvisited.remove(&cur);
        visited.insert(cur, cur_cost);
    }

    *visited.get(&goal).unwrap()
}

fn get_unvisited_neighbours(cur: (usize, usize), visited: &HashMap<(usize, usize), u32>, max: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::with_capacity(4);

    if cur.0 > 0 && !visited.contains_key(&(cur.0 - 1, cur.1)) {
        neighbours.push((cur.0 - 1, cur.1));
    }
    if cur.1 > 0 && !visited.contains_key(&(cur.0, cur.1 - 1)) {
        neighbours.push((cur.0, cur.1 - 1));
    }
    if cur.0 < max && !visited.contains_key(&(cur.0 + 1, cur.1)) {
        neighbours.push((cur.0 + 1, cur.1));
    }
    if cur.1 < max && !visited.contains_key(&(cur.0, cur.1 + 1)) {
        neighbours.push((cur.0, cur.1 + 1));
    }

    neighbours
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::iter;

#[derive(Eq, PartialEq, Clone)]
struct MapNode {
    pos: (usize, usize),
    distance: usize,
}

impl Ord for MapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for MapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

fn cheapest_path(grid: &[Vec<u32>]) -> u32 {
    let grid_size = grid.len();

    // Implementing A* algorithm
    // The heiruestic algorithm will be a simple hanhattan distance to the end
    // As if it could take a direct path of 1's there that would be the distance
    // Creating a map containing visitable nodes
    let mut open_nodes: BinaryHeap<MapNode> = BinaryHeap::new();
    open_nodes.push(MapNode {
        pos: (0, 0),
        distance: grid_size * 2 - 2,
    });

    let mut lowest_dist: HashMap<(usize, usize), Option<u32>> =
        HashMap::with_capacity(grid_size * grid_size);
    lowest_dist.insert((0, 0), Some(0));
    for x in 0..grid_size {
        for y in 0..grid_size {
            if x != 0 || y != 0 {
                lowest_dist.insert((x, y), None);
            }
        }
    }

    let goal = (grid_size - 1, grid_size - 1);

    while let Some(cur) = open_nodes.pop() {
        if cur.pos == goal {
            return lowest_dist.get(&cur.pos).unwrap().unwrap();
        }

        for neighbour in get_neighbours(cur.pos, grid_size - 1) {
            let travel_score =
                lowest_dist.get(&cur.pos).unwrap().unwrap() + grid[neighbour.0][neighbour.1];
            // If there isn't currently a defined lowest distance, or if the defined lowest distance is greater than the trip from cur to neighbour, set that
            if lowest_dist
                .get(&neighbour)
                .unwrap()
                .map_or(true, |cur_travel| cur_travel > travel_score)
            {
                lowest_dist.insert(neighbour, Some(travel_score));
                // If neighbour isn't in open_nodes, add it
                let neighbour_node = open_nodes.iter().find(|node| node.pos == neighbour);
                let new_node = MapNode {
                    pos: neighbour,
                    distance: travel_score as usize + grid_size * 2 - neighbour.0 - neighbour.1 - 2,
                };
                match neighbour_node {
                    None => open_nodes.push(new_node),
                    Some(node) => {
                        if new_node > *node {
                            let node = node.clone();
                            // Removing the old one and adding the new one
                            open_nodes = open_nodes
                                .drain()
                                .filter(|x| node != *x)
                                .chain(iter::once(new_node))
                                .collect();
                        }
                    }
                }
            }
        }
    }

    panic!("Failed to find path");
}

fn get_neighbours(cur: (usize, usize), max: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::with_capacity(4);

    if cur.0 > 0 {
        neighbours.push((cur.0 - 1, cur.1));
    }
    if cur.1 > 0 {
        neighbours.push((cur.0, cur.1 - 1));
    }
    if cur.0 < max {
        neighbours.push((cur.0 + 1, cur.1));
    }
    if cur.1 < max {
        neighbours.push((cur.0, cur.1 + 1));
    }

    neighbours
}

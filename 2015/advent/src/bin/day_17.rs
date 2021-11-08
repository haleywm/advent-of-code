use itertools::Itertools;
use std::env;

fn main() {
    let containers: Vec<u32> = advent::line_iter("input/day_17.txt")
        .expect("Unable to open input file")
        .map(|line| {
            let line = line.unwrap();
            line.parse().unwrap()
        })
        .collect();
    let goal: u32 = env::args().nth(1).and_then(|x| x.parse().ok()).unwrap_or(25);

    let valid_containers = containers
        .iter()
        .powerset()
        .filter(|x| x.iter().cloned().sum::<u32>() == goal);

    let result_one = valid_containers.clone()
        .count();

    println!("{}", result_one);

    let min_counters = valid_containers
        .clone()
        .map(|x| x.len())
        .min()
        .unwrap();
    
    let result_two = valid_containers
        .filter(|x| x.len() == min_counters)
        .count();
    
    println!("{}", result_two);
}

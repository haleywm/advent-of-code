use std::collections::hash_map::HashMap;
use regex::Regex;
use itertools::{Itertools, MinMaxResult};

fn main() {
    let mut name_map: HashMap<String, usize> = HashMap::new();
    let mut dist_map: HashMap<(usize, usize), usize> = HashMap::new();
    let re = Regex::new(r"^(.+) to (.+) = (\d+)$").unwrap();

    advent::line_iter("input/day_9.txt")
        .expect("Failed to open file")
        .for_each(|line| {
            let line = line.unwrap();
            let res = re.captures(&line).expect("Failed to parse line");
            let a = res.get(1).unwrap().as_str();
            let next_id = name_map.len();
            let a_id = *name_map.entry(a.to_owned()).or_insert(next_id);
            let b = res.get(2).unwrap().as_str();
            let next_id = name_map.len();
            let b_id = *name_map.entry(b.to_owned()).or_insert(next_id);
            let dist = res.get(3).unwrap().as_str().parse::<usize>().unwrap();

            dist_map.insert((a_id, b_id), dist);
            dist_map.insert((b_id, a_id), dist);
        });
    
    println!("{}", name_map.len());
    println!("{:?}", name_map);
    println!("{:?}", dist_map);
    // Bad, naieve algorithm, seeing if it works:
    if let MinMaxResult::MinMax::<usize>(min, max) = (0..name_map.len())
        .permutations(name_map.len())
        .map(|order| {
            order.windows(2)
                .map(|step| dist_map.get(&(step[0], step[1])).unwrap())
                .sum()
        })
        .minmax() {
        println!("Shortest Path: {}", min);
        println!("Longest Path: {}", max);
    }
    else {
        panic!("Didn't find at least 2 paths");
    }
}
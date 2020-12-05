use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let splitter: Regex = Regex::new(r"^(.+)\)(.+)$").unwrap();
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let vals: Vec<(String, String)> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Couldn't read line"))
        .map(|x| {
            let result = splitter.captures(x.trim()).unwrap();
            (result[1].to_owned(), result[2].to_owned())
        })
        .collect();
    
    // Inverting the graph for this problem
    let mut graph: HashMap<&str, Option<&str>> = HashMap::new();
    for (from, to) in vals.iter() {
        // Creating new nodes if needed
        // Each node will contain it's parent
        graph.entry(&from).or_insert(None);
        let to = graph.entry(&to).or_insert(None);
        if to.is_some() {
            panic!("Multiple parents");
        }
        *to = Some(from);
    }
    // Lastly, recursively exploring the tree
    // If there are loops this will end poorly don't do that
    let mut start = graph.get("YOU").unwrap();
    let mut start_dist = 0;
    let mut result = None;

    // To get the path, we need to find the shared parent
    'search: while start.is_some() {
        start = graph.get(start.unwrap()).unwrap();
        start_dist += 1;
        
        let mut dest = graph.get("SAN").unwrap();
        let mut dest_dist = 0;
        while dest.is_some() {
            dest = graph.get(dest.unwrap()).unwrap();
            dest_dist += 1;
            if dest == start {
                // Found it
                result = Some(start_dist + dest_dist);
                break 'search;
            }
        }
    }

    println!("{}", result.unwrap_or(-1));
}

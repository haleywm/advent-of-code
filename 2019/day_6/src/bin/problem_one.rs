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
    
    // Making my own graph using a hash table, where each item contains a list of it's children
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for (from, to) in vals.iter() {
        // Creating new nodes if needed
        // Each node will contain a list of their children
        graph.entry(&to).or_insert(Vec::new());
        let from = graph.entry(&from).or_insert(Vec::new());
        from.push(to);
    }
    // Lastly, recursively exploring the tree
    // If there are loops this will end poorly don't do that
    let result = orbit_count(&graph, graph.get("COM").unwrap(), 0);

    println!("{}", result);
}

fn orbit_count(table: &HashMap<&str, Vec<&str>>, cur: &Vec<&str>, depth: i32) -> i32 {
    let mut total = depth;
    for child in cur {
        total += orbit_count(&table, table.get(child).unwrap(), depth + 1);
    }
    total
}

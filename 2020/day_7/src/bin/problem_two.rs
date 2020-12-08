use std::fs;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    // Doing structure in the opposite direction this time around
    // If only there were a graph implementation on rust that didn't suck
    // Maybe I'll make my own one day
    let parent_match = Regex::new(r"^(.+) bags contain ").unwrap();
    let child_match = Regex::new(r"(\d+) ([^.,]+) bags?").unwrap();

    // Creating a graph map, as well as a vector to store the strings
    let mut graph: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for line in lines {
        let line = line.expect("Couldn't read line");
        let parent = parent_match.captures(&line).unwrap();
        let parent_name = &parent[1];
        graph.entry(parent_name.to_owned()).or_insert_with(|| Vec::new());

        let children = child_match.captures_iter(
            &line[parent.get(0).unwrap().end()..]
        );
        for child in children {
            // For each child, create an entry if one doesn't exist, and then append the parent
            if !graph.contains_key(&child[2]) {
                graph.insert(child[2].to_owned(), Vec::new());
            }
            // Doing this every single time to not violate memory rules, hopefully gets optimized to be better
            let parent_content = graph.get_mut(parent_name).unwrap();
            parent_content.push((child[2].to_owned(), child[1].parse().unwrap()));
            //println!("{}: {}", parent_content.last().unwrap().0, parent_content.last().unwrap().1)
        }
    }

    let result = rec_read(&graph, "shiny gold") - 1;
    println!("{}", result);
}

fn rec_read(table: &HashMap<String, Vec<(String, i32)>>, bag: &str) -> i32
{
    let items = table.get(bag).unwrap();
    let mut total = 1;
    for (bag, count) in items {
        total += *count * rec_read(table, bag);
    }

    total
}
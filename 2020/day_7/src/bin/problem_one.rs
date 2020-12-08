use std::fs;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    // Making a few assumptions about the data set here
    // Bag types probably aren't specified multiple times
    // Will use the easiest implementation instead of making something more complex and hoping it will do most of the work for part 2
    // Will create a hash map of bag names, each containing a vector of bag names that can hold them
    // Then, at the end, I'll iterate through and collect the names into a new hash set to avoid multiple items showing up, and get the count
    let parent_match = Regex::new(r"^(.+) bags contain ").unwrap();
    let child_match = Regex::new(r"\d+ ([^,.]+) bags?").unwrap();

    // Creating a graph map, as well as a vector to store the strings
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let line = line.expect("Couldn't read line");
        let parent = parent_match.captures(&line).unwrap();

        let parent_name = &parent[1];

        if !graph.contains_key(parent_name) {
            graph.insert(parent_name.to_owned(), Vec::new());
        }

        let children = child_match.captures_iter(
            &line[parent.get(0).unwrap().end()..]
        );
        for child in children {
            // For each child, create an entry if one doesn't exist, and then append the parent
            //let child = &child[1];
            let child = graph.entry(child[1].to_owned()).or_insert_with(|| Vec::new());
            child.push(parent_name.to_owned());
        }
    }
    let to_fit = graph.get("shiny gold").unwrap();

    let result = rec_read(&graph, to_fit);
    println!("{}", result);
}

fn rec_read(table: &HashMap<String, Vec<String>>, list: &Vec<String>) -> usize
{
    let mut full_list: HashSet<&str> = HashSet::new();

    let mut to_explore: Vec<&str> = Vec::new();
    for item in list {
        to_explore.push(item);
    }
    while let Some(next) = to_explore.pop() {
        let (name, next_list) = table.get_key_value(next).unwrap();
        full_list.insert(name);
        for items in next_list {
            /*if !full_list.contains(items) {
                to_explore.push(items);
            }*/
            to_explore.push(items);
        }
    }
    full_list.len()
}
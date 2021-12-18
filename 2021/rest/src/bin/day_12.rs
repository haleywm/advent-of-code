use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let matcher = Regex::new(r"^(\w+)-(\w+)$").unwrap();
    let file = fs::File::open("input/day_12.txt").expect("Invalid Filename");
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let cap = matcher.captures(&line).expect("Invalid line");
        let from = cap.get(1).unwrap().as_str();
        let to = cap.get(2).unwrap().as_str();

        let from_list = map.entry(from.to_owned()).or_insert_with(Vec::new);
        from_list.push(to.to_owned());

        let to_list = map.entry(to.to_owned()).or_insert_with(Vec::new);
        to_list.push(from.to_owned());
    }

    println!("{}", recursive_explore_p1(HashSet::new(), "start", &map));
    println!(
        "{}",
        recursive_explore_p2(HashSet::new(), "start", &map, false)
    );
}

fn is_small(name: &str) -> bool {
    name.chars().any(char::is_lowercase)
}

// Recursively explores a cave system, returning the total number of possible paths
// Will recurse forever if it's possible to have infinite paths
fn recursive_explore_p1(
    mut visited: HashSet<String>,
    cur: &str,
    map: &HashMap<String, Vec<String>>,
) -> usize {
    if cur == "end" {
        return 1;
    }

    if is_small(cur) {
        visited.insert(cur.to_owned());
    }

    let options = map.get(cur).unwrap();

    options
        .iter()
        .filter(|cave| {
            // Only go if the cave is large, or if it's small and unvisited
            !is_small(cave) || !visited.contains(*cave)
        })
        .map(|cave| recursive_explore_p1(visited.clone(), cave, map))
        .sum()
}

// Recursively explores a cave system, returning the total number of possible paths
// Will recurse forever if it's possible to have infinite paths
fn recursive_explore_p2(
    mut visited: HashSet<String>,
    cur: &str,
    map: &HashMap<String, Vec<String>>,
    small_visited: bool,
) -> usize {
    if cur == "end" {
        return 1;
    }

    if is_small(cur) {
        visited.insert(cur.to_owned());
    }

    let options = map.get(cur).unwrap();

    options
        .iter()
        .filter(|cave| {
            // Only go if the cave is large, or if it's small and unvisited
            (!is_small(cave) || !(visited.contains(*cave) && small_visited)) && *cave != "start"
        })
        .map(|cave| {
            recursive_explore_p2(
                visited.clone(),
                cave,
                map,
                small_visited || visited.contains(cave),
            )
        })
        .sum()
}

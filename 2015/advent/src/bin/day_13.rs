use std::collections::{HashMap, HashSet};
use regex::Regex;
use itertools::Itertools;

fn main() {
    let re = Regex::new(r"^(.+) would (gain|lose) (\d+) happiness units by sitting next to (.+)\.$").unwrap();
    let mut all_guests: HashSet<String> = HashSet::new();
    
    let relations: HashMap<(String, String), i64> = advent::line_iter("input/day_13.txt")
        .expect("Unable to open input file")
        .map(|line| {
            let line = line.unwrap();
            let cap = re.captures(&line).expect("Error matching regex");

            let from = cap.get(1).unwrap().as_str().to_owned();
            let to = cap.get(4).unwrap().as_str().to_owned();
            let mut change = cap.get(3).unwrap().as_str().parse().unwrap();
            match cap.get(2).unwrap().as_str() {
                "gain" => {},
                "lose" => change *= -1,
                _ => panic!("Unrecognised value"),
            };

            // Inserting names into hashset if they aren't already
            all_guests.insert(from.clone());
            all_guests.insert(to.clone());

            ((from, to), change)
        })
        .collect();
    
    let result_one = get_result(&all_guests, &relations);
    println!("{}", result_one);

    all_guests.insert(String::from("Me"));

    let result_two = get_result(&all_guests, &relations);
    println!("{}", result_two);
}

fn get_result(all_guests: &HashSet<String>, relations: &HashMap<(String, String), i64>) -> i64 {
    // Lazy bruteforce method :)
    all_guests.iter()
        .permutations(all_guests.len())
        .map(|guests| {
            guests.windows(2)
                .map(|pair| {
                    relations.get(&(pair[0].to_owned(), pair[1].to_owned())).unwrap_or(&0) +
                    relations.get(&(pair[1].to_owned(), pair[0].to_owned())).unwrap_or(&0)
                })
                .sum::<i64>() +
                relations.get(&(guests[0].to_owned(), guests[guests.len() - 1].to_owned())).unwrap_or(&0) +
                relations.get(&(guests[guests.len() - 1].to_owned(), guests[0].to_owned())).unwrap_or(&0)
        })
        .max()
        .unwrap()
}

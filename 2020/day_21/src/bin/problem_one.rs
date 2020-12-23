use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
struct Food {
    ing: HashSet<String>,
    alg: HashSet<String>,
}

impl Food {
    pub fn new() -> Food {
        Food { ing: HashSet::new(), alg: HashSet::new() }
    }
}

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    let separator = Regex::new(r" \(contains ").unwrap();
    let word_cap = Regex::new(r"\w+").unwrap();

    // Each item in the input will be stored in a vector containing a list of ingredients on one side, and allergens on the other
    let mut foods: Vec<Food> = Vec::new();
    // This will contain every ingredient, with it's value starting as true, and set to false if considered unsafe
    let mut good_stuff: HashSet<String> = HashSet::new();
    let mut bad_stuff: HashSet<String> = HashSet::new();

    for line in lines {
        let line = line.unwrap();
        
        let sep = separator.find(&line).unwrap();
        let mut food = Food::new();

        for comp in word_cap.captures_iter(&line[..sep.start()]) {
            good_stuff.insert(comp[0].to_owned());
            food.ing.insert(comp[0].to_owned());
        }

        for nasty in word_cap.captures_iter(&line[sep.end()..]) {
            bad_stuff.insert(nasty[0].to_owned());
            food.alg.insert(nasty[0].to_owned());
        }
        foods.push(food);
    }

    //println!("{:?}", foods);
    //println!("{:?}", good_stuff);
    //println!("{:?}", bad_stuff);

    // The way foods can be proven safe is by looking at the set of all foods with each allergen.
    // Find the list of ingredients that are common to every food on this list
    // The are unsafe, and should be removed from the good stuff list
    for nasty in bad_stuff {
        let mut affected_list = foods.iter()
            .filter(|x| x.alg.contains(&nasty))
            .map(|x| &x.ing);
        let first_food = affected_list.next().unwrap();
        let rest: Vec<&HashSet<String>> = affected_list.collect();
        
        for notsafe in first_food.iter().filter(|item| rest.iter().all(|comp| comp.contains(*item))) {
            good_stuff.remove(notsafe);
        }
    }
    //println!("{:?}", good_stuff);
    // Lastly, counting how many times these occur
    let result: usize = foods.iter()
        .map(|x| {
            good_stuff.intersection(&x.ing).count()
        })
        .sum();
    
    println!("{}", result);
}

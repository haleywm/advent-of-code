use std::fs;
use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};
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
    //let mut unsafe_stuff: HashMap<String, Option<String>> = HashMap::new();
    let mut unsafe_stuff: HashMap<String, String> = HashMap::new();
    let mut bad_stuff: HashSet<String> = HashSet::new();

    for line in lines {
        let line = line.unwrap();
        
        let sep = separator.find(&line).unwrap();
        let mut food = Food::new();

        for comp in word_cap.captures_iter(&line[..sep.start()]) {
            //good_stuff.insert(comp[0].to_owned());
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
    while bad_stuff.len() > 0 {
        for nasty in bad_stuff.iter() {
            let mut affected_list = foods.iter()
                .filter(|x| x.alg.contains(nasty))
                .map(|x| &x.ing);
            let first_food = affected_list.next().unwrap();
            let rest: Vec<&HashSet<String>> = affected_list.collect();
            
            let notsafe: Vec<&String> = first_food.iter()
                .filter(|item| !unsafe_stuff.contains_key(*item) && rest.iter().all(|comp| comp.contains(*item)))
                .collect();
                //unsafe_stuff.insert(notsafe.to_owned(), None);
            // If there is only one unshared item here, use that, otherwise keep the item around for next time
            assert_ne!(notsafe.len(), 0);
            if notsafe.len() == 1 {
                unsafe_stuff.insert(notsafe[0].to_owned(), nasty.to_owned());
            }
        }
        for (_, key) in unsafe_stuff.iter() {
            // Taking items we already know the identity out of the list until it's empty
            bad_stuff.remove(key);
        }
    }
    
    
    // Lastly, export the hashmap and sort by the key value alphabetically
    let mut result: Vec<(String, String)> = unsafe_stuff.into_iter().collect();
    result.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut result = result.iter();
    print!("{}", result.next().unwrap().0);
    for (key, _) in result {
        print!(",{}", key);
    }
    println!();
}

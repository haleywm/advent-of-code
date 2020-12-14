use num::integer::Integer;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file).lines();

    let mut processes: HashMap<String, (Vec<(String, i64)>, i64)> = HashMap::new();

    let recipe_parser = Regex::new(r"\b(\d+) ([^\s,]+),? ").unwrap();
    let result_parser = Regex::new(r"=> (\d+) ([^\s]+)$").unwrap();

    for line in lines {
        let line = line.unwrap();

        let mut ingredients: Vec<(String, i64)> = Vec::new();
        for ingredient in recipe_parser.captures_iter(&line) {
            ingredients.push((ingredient[2].to_owned(), ingredient[1].parse().unwrap()));
        }
        let result = result_parser.captures(&line).unwrap();
        let amount: i64 = result[1].parse().unwrap();
        let name = result[2].to_owned();

        processes.insert(name, (ingredients, amount));
    }

    // Now that the input has been parsed into a map, I have to walk from FUEL TO ORE
    const FROM: &str = "ORE";
    const TO: &str = "FUEL";
    // Using a hashmap to keep track of how much is needed for everything
    // The first
    let needed_used_amount: HashMap<String, (i64, i64)> = HashMap::new();

    let needed_used_amount = rec_add(needed_used_amount, &processes, TO, 1);
    //println!("{:?}", needed_used_amount);
    let result = needed_used_amount.get(FROM).unwrap().0;
    println!("{}", result);
}

fn rec_add(
    mut nua: HashMap<String, (i64, i64)>,
    recipies: &HashMap<String, (Vec<(String, i64)>, i64)>,
    to_eval: &str,
    needed: i64,
) -> HashMap<String, (i64, i64)> {
    let (cur_used, cur_amount) = nua.entry(to_eval.to_owned()).or_insert((0, 0));
    let to_create = needed - (*cur_amount - *cur_used);

    // Increasing the amount used by the amount required
    *cur_used += needed;

    // No point working this out if existing stock will cover it
    if to_create > 0 {
        let components = recipies.get(to_eval);
        // If components returns none, it probably means we reached the top level
        if let Some((comps, output_amount)) = components {
            // Getting the smallest amount that can be created that's a mulitple of what we can produce
            let actual_output_amount = to_create.next_multiple_of(&output_amount);
            // Get the necessary multiplier
            let mult = actual_output_amount / output_amount;
            // Increasing the stockpile by the amount produced
            *cur_amount += actual_output_amount;
            for (item, item_num) in comps.iter() {
                let item_num = item_num * mult;
                // By passing ownership to the recursive method, and then returning it back I can avoid violating the single mut rule
                nua = rec_add(nua, recipies, &item, item_num);
            }
        }
    }

    nua
}

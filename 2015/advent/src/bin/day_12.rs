use itertools::Itertools;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Reading the file and passing it to serde
    let file = File::open("input/day_12.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    
    let json: Value = serde_json::from_reader(reader).expect("Unable to parse json");

    let total_one = walk_json(&json, false);
    println!("{}", total_one);

    let total_two = walk_json(&json, true);
    println!("{}", total_two);
}

fn walk_json(json: &Value, ignore_red: bool) -> i64 {
    // Walks the json file, summing all numbers
    let mut total = 0;

    match json {
        Value::Number(num) => total += num.as_i64().unwrap(),
        Value::Array(children) => {
            for val in children {
                total += walk_json(val, ignore_red);
            }
        },
        Value::Object(children) => {
            if !(ignore_red && children.values().contains(&Value::String(String::from("red")))) {
                // Ignore objects with a proterty equal to red
                for val in children.values() {
                    total += walk_json(val, ignore_red);
                }
            }
        },
        _ => {} // Don't care about other possibilities
    }

    total
}

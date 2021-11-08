use std::collections::HashMap;
use regex::Regex;

type Properties = HashMap<String, u32>;
fn main() {
    let start_re = Regex::new(r"^Sue (\d+): ").unwrap();
    let property_match = Regex::new(r"(\w+): (\d+)").unwrap();

    let target: Properties = HashMap::from([
        (String::from("children"), 3),
        (String::from("cats"), 7),
        (String::from("samoyeds"), 2),
        (String::from("pomeranians"), 3),
        (String::from("akitas"), 0),
        (String::from("vizslas"), 0),
        (String::from("goldfish"), 5),
        (String::from("trees"), 3),
        (String::from("cars"), 2),
        (String::from("perfumes"), 1)
    ]);
    let sues: Vec<(u32, Properties)> = advent::line_iter("input/day_16.txt")
        .expect("Unable to open input file")
        .map(|line| {
            let line = line.unwrap();
            let cap = start_re.captures(&line).expect("Unable to parse line");
            let num: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let end = cap.get(0).unwrap().end();
            let properties: Properties = property_match.captures_iter(&line[end..])
                .map(|cap| {
                    let name = cap.get(1).unwrap().as_str().to_owned();
                    let count = cap.get(2).unwrap().as_str().parse().unwrap();
                    (name, count)
                })
                .collect();
            (num, properties)
        })
        .collect();
    
    let sue_one = sues
        .iter()
        .find(|s| {
            let dict = &s.1;
            for (k, v) in dict.iter() {
                if *target.get(k).unwrap() != *v {
                    // Not it
                    return false;
                }
            }
            // If it passed it must fit
            return true;
        })
        .expect("Couldn't find a match").0;
    println!("Sue Number {}", sue_one);
    
    let sue_two = sues
        .iter()
        .find(|s| {
            let dict = &s.1;
            for(k, v) in dict.iter() {
                match k.as_str() {
                    "cats" | "trees" => {
                        if *target.get(k).unwrap() >= *v {
                            return false;
                        }
                    },
                    "pomeranians" | "goldfish" => {
                        if *target.get(k).unwrap() <= *v {
                            return false;
                        }
                    }
                    _ => {
                        if *target.get(k).unwrap() != *v {
                            return false;
                        }
                    }
                }
            }
            // If it passed then it must fit
            true
        })
        .unwrap()
        .0;
    
    println!("Sue Number {}", sue_two);
}

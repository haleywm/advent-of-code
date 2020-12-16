use std::fs;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file)
        .lines();
    
    let rule_parse = Regex::new(r".+: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    
    let mut rules: Vec<(i32, i32)> = Vec::new();
    loop {
        let next_line = lines.next().unwrap().unwrap();
        if next_line.len() == 0 {
            // Found a blank line, the end of rules
            // Disposing of the next 3 lines
            for _ in 0..4 {
                lines.next();
            }
            break;
        }
        let values = rule_parse.captures(&next_line).unwrap();
        rules.push(
            (values[1].parse().unwrap(), values[2].parse().unwrap())
        );
        rules.push(
            (values[3].parse().unwrap(), values[4].parse().unwrap())
        );
    }
    println!("{} Rules", rules.len());

    // Iterating over remaining lines
    let result: i32 = lines.map(|x| {
        // The first val of cur is the number of valid tickets, the second is the total amount
        let ticket = x.unwrap();
        //println!("{}", ticket);
        ticket.split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .map(|val| {
                for rule in &rules {
                    if val >= rule.0 && val <= rule.1 {
                        // Found a matching set
                        return 0;
                    }
                }
                // No match
                val
            }).sum::<i32>()
    }).sum();
    println!("{}", result);
}

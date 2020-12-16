use std::fs;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file)
        .lines();
    
    let rule_parse = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    
    let mut my_ticket: Vec<i64> = Vec::new();
    // First range (min, max), second range, name, matched row (default 0)
    let mut rules: Vec<((i64, i64), (i64, i64), String, usize)> = Vec::new();
    loop {
        let next_line = lines.next().unwrap().unwrap();
        if next_line.len() == 0 {
            // Found a blank line, the end of rules
            // Getting my ticket, and disposing of the rest
            lines.next();
            let mine = lines.next().unwrap().unwrap();
            for num in mine.split(',') {
                let num = num.parse().unwrap();
                my_ticket.push(num);
            }
            lines.next();
            lines.next();
            break;
        }
        let values = rule_parse.captures(&next_line).unwrap();
        rules.push(
            (
                (values[2].parse().unwrap(), values[3].parse().unwrap()),
                (values[4].parse().unwrap(), values[5].parse().unwrap()),
                values[1].to_owned(),
                0
            )
        );
    }
    println!("{} Rules", rules.len());

    // Iterating over remaining lines
    let valid_tickets: Vec<Vec<i64>> = lines.filter_map(|x| {
        // The first val of cur is the number of valid tickets, the second is the total amount
        let ticket = x.unwrap();
        //println!("{}", ticket);
        let ticket: Vec<i64> = ticket.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        for val in &ticket {
            let mut matches = false;
            for rule in &rules {
                if (*val >= rule.0.0 && *val <= rule.0.1) || (*val >= rule.1.0 && *val <= rule.1.1) {
                    // Found a matching set
                    matches = true;
                    break;
                }
            }
            if !matches {
                return None;
            }
        }
        Some(ticket)
    }).collect();
    //println!("{:?}", valid_tickets);
    // Now that invalid items have been filtered out, I must work out what matches what
    let rules_len = rules.len();
    let mut matched_rows: Vec<Vec<usize>> = vec![Vec::new(); rules_len];
    for (i, to_match) in rules.iter_mut().enumerate() {
        // Iterate through each item until I find it
        let mut match_made = false;
        for col in 0..rules_len {
            //if matched_rows.contains(&i) {
            //    // Don't want to test something already matched
            //    continue;
            //}
            let mut valid = true;
            for ticket in valid_tickets.iter() {
                if (ticket[col] < to_match.0.0 || ticket[col] > to_match.0.1) && (ticket[col] < to_match.1.0 || ticket[col] > to_match.1.1) {
                    // Ticket is invalid
                    valid = false;
                    break;
                }
            }
            if valid {
                // Found a match
                //println!("{}, {}", col, i);
                matched_rows[col].push(i);
                match_made = true;
            }
        }
        if !match_made {
            panic!("No match for {:?}", to_match);
        }
    }
    // Then, unwrapping to determine the possible rules.
    // Assuming that there will be at least 1 item with only 1 possibility, and once that has been discounted from the rest there will be another with 1, and so forth
    for _ in 0..rules_len {
        let (col, rule_index) = matched_rows
            .iter()
            .enumerate()
            .find(|(_, x)| x.len() == 1)
            .unwrap();
        let val = rule_index[0];
        rules[val].3 = col;
        println!("{}, {}", val, col);
        for rule in matched_rows.iter_mut() {
            rule.retain(|x| *x != val);
        }
    }

    let result: i64 = rules.iter()
        .filter(|rule| rule.2.starts_with("departure"))
        .inspect(|rule| println!("{:?}", rule))
        .map(|rule| my_ticket[rule.3])
        .inspect(|x| println!("{}", x))
        .product();
    println!("{}", result);
}

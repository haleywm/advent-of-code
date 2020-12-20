use std::fs;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

// The first vector represents every 'or rule', so only of the contained vectors must be true.
// The second vector contains every other rule number that must be satisfied in order.
type RuleSet = Vec<Vec<usize>>;
type RuleBook = HashMap<usize, Rule>;

enum Rule {
    // Rule requests a single character
    Char(char),
    // Rule requests a set of other rules
    Set(RuleSet),
}

use Rule::*;

//type Rule = Vec<i32>;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file)
        .lines();

    // Matches rules that specify a single character
    let match_char = Regex::new(r#"^(\d+): "([a-z])"$"#).unwrap();
    // Matches the beginning of rules, with the rule number
    let match_start = Regex::new(r"^(\d+):").unwrap();
    // Matches every other value in the rule after the rule number (either a number or |)
    let match_after = Regex::new(r" (\d+|\|)").unwrap();

    let mut rules: RuleBook = HashMap::new();

    loop {
        let rule = lines.next().unwrap().unwrap();
        if rule.len() == 0 {
            // Found an empty line, signifying the end of rules. After this comes the strings to test
            break;
        }
        match match_char.captures(&rule) {
            Some(result) => {
                // The rule is a Char
                let pos = result[1].parse().unwrap();
                // This is still the easiest way to get the first char of a string
                let val = Char(result[2].chars().next().unwrap());
                rules.insert(pos, val);
            }
            None => {
                // The rule is therefore a Set
                let start = match_start.captures(&rule).unwrap();
                let pos = start[1].parse().unwrap();

                let mut rule_options = Vec::new();
                rule_options.push(Vec::new());

                for token in match_after.captures_iter(
                    &rule[start.get(0).unwrap().end()..]
                ) {
                    // Start iteration after the rule number so that any number of space seperated rules can be matched
                    match token[1].parse::<usize>() {
                        // Another number, appending to the last rule set in the list
                        Ok(num) => rule_options.last_mut().unwrap().push(num),
                        // Item must have been |, inserting a new ruleset to add numbers to
                        Err(_) => rule_options.push(Vec::new()),
                    }
                }

                rules.insert(pos, Set(rule_options));
            }
        }
    }

    // The rules have been parsed, now to check if each line meets the requirements
    let result = lines
        .filter(|line| {
            let line = line.as_ref().unwrap();

            meets_rule(&rules, 0, line).map_or(false, |x| x.contains(&line.len()))
        })
        //.inspect(|x| println!("{}", x.as_ref().unwrap()))
        .count();
    println!("{}", result);
}

fn meets_rule(rules: &RuleBook, num: usize, comp: &str) -> Option<Vec<usize>> {
    let rule = rules.get(&num).expect("Unknown rule number");
    match rule {
        Char(letter) => {
            // Base case
            // Just need to check if the given string starts with the required letter
            if comp.starts_with(*letter) {
                return Some(vec![1]);
            }
            else {
                return None;
            }
        }
        Set(options) => {
            // Find if any of the options in this ruleset are valid, using recursion to resolve child rules
            // If a ruleset contains a loop this will cause the program to freeze
            let mut results = Vec::new();
            'rule_check: for set in options {
                // If this set is valid, return that, otherwise move on and try other options until they are exhausted
                let mut pos = vec![0];
                for val in set {
                    let mut new_pos = Vec::new();
                    for i in 0..pos.len() {
                        match meets_rule(&rules, *val, &comp[pos[i]..]) {
                            Some(options) => {
                                // This rule matched a certain number of chars, as indicated by the contents of options
                                //pos[i] += len;
                                for len in options {
                                    new_pos.push(pos[i] + len);
                                }
                            }
                            None => {
                                // This rule didn't match
                            }
                        }
                    }
                    // Replace the old positions with next turns positions
                    pos = new_pos;
                    // Then give up on this loop and move on to the next if there's no more options
                    if pos.len() == 0 {
                        continue 'rule_check;
                    }
                }
                // Values matched this set
                results.append(&mut pos);
            }
            if results.len() > 0 {
                return Some(results);
            }
            else {
                return None;
            }
        }
    }
}

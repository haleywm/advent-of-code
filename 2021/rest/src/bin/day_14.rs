use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

type RuleSet = HashMap<(char, char), char>;

fn main() {
    let parse_template = Regex::new(r"^([A-Z])([A-Z]) -> ([A-Z])$").unwrap();

    let file = fs::File::open("input/day_14.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file).lines().map(Result::unwrap);

    // Getting the first line for our polymer
    let polymer = lines.next().unwrap();
    // Skipping the next line (Should be blank)
    lines.next();

    let rules: RuleSet = lines
        .map(|line| {
            let cap = parse_template.captures(&line).unwrap();
            let first = cap.get(1).unwrap().as_str().chars().next().unwrap();
            let second = cap.get(2).unwrap().as_str().chars().next().unwrap();
            let insert = cap.get(3).unwrap().as_str().chars().next().unwrap();
            ((first, second), insert)
        })
        .collect();

    let polymer = polymer.chars().collect::<Vec<_>>();

    // Part 1
    println!("{}", perform_polymer_steps(polymer.as_slice(), &rules, 10));
    // Part 2
    println!("{}", perform_polymer_steps(polymer.as_slice(), &rules, 40));
}

fn perform_polymer_steps(polymer_slice: &[char], rules: &RuleSet, count: usize) -> usize {
    let mut char_count = polymer_slice.iter().counts_by(char::to_owned);
    let mut pair_count = polymer_slice.windows(2).counts_by(|x| (x[0], x[1]));

    for _ in 0..count {
        let mut pair_adjust: HashMap<(char, char), i64> = HashMap::new();
        for (&(a, b), &res) in rules.iter() {
            let cur_pairs = *pair_count.get(&(a, b)).unwrap_or(&0);
            let cur_pairs_signed = cur_pairs as i64;
            *pair_adjust.entry((a, b)).or_default() -= cur_pairs_signed;
            *pair_adjust.entry((a, res)).or_default() += cur_pairs_signed;
            *pair_adjust.entry((res, b)).or_default() += cur_pairs_signed;
            *char_count.entry(res).or_default() += cur_pairs;
        }
        // Merging adjustment
        for (pair, adjustment) in pair_adjust.into_iter() {
            let cur = *pair_count.get(&pair).unwrap_or(&0) as i64;
            let new = (cur + adjustment) as usize;
            pair_count.insert(pair, new);
        }
    }

    // Get frequency
    let values = char_count.values().minmax();

    if let MinMax(low, high) = values {
        high - low
    } else {
        panic!("Unable to find highest and lowest");
    }
}

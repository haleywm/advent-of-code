use std::fs;
use std::io::{self, BufRead};

const STEPS: usize = 50000000000;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file)
        .lines();
    
    // Parsing first line
    let first = lines.next().unwrap().unwrap();
    let mut pots: Vec<bool> = first[15..].chars()
        .map(|x| x == '#')
        .collect();

    // Ignoring next line
    lines.next();
    // Then, for each remaining line parsing it as a rule
    let mut will_grow: Vec<[bool; 5]> = Vec::new();

    for rule in lines {
        let rule = rule.unwrap();
        let makes_tree = rule.chars().rev().next().unwrap() == '#';
        // Only add the rule if it makes a tree, no point explicitly parsing ones that don't
        if makes_tree {
            let mut pattern = rule.chars();
            // Rust add a system to duplicate methods for arrays I'm begging you
            let items = [
                pattern.next().unwrap() == '#',
                pattern.next().unwrap() == '#',
                pattern.next().unwrap() == '#',
                pattern.next().unwrap() == '#',
                pattern.next().unwrap() == '#',
            ];
            will_grow.push(items);
        }
    }

    let mut total_offset = 0;

    let mut last_result = 0;
    let mut last_increase = 0;
    let mut pattern_for = 0;

    let mut i = 0;

    for _ in 0..STEPS {
        let mut new_pots: Vec<bool> = Vec::with_capacity(pots.len() + 4);
        let mut started = false;
        let mut started_at = 0;
        for i in 0..pots.len() + 4 {
            // pots is offset 2 from new_pots
            let mut cur_sur = [false; 5];
            for off in 0..5 {
                // Doing a saturating sub as val must be at least 2 to do something, and reduces typecasting
                let rel_off = (i + off).saturating_sub(2);
                if rel_off >= 2 && rel_off < pots.len() + 2 {
                    // Val is within bounds. If val isn't in bounds, the default value false is correct
                    cur_sur[off] = pots[rel_off - 2];
                }
            }
            let grow = will_grow.contains(&cur_sur);
            if !started && grow {
                // Reached the start of stuff happening
                started = true;
                started_at = i as i32 - 2;
            }
            if started {
                new_pots.push(grow);
            }
        }
        
        // Removing unneeded items off the end
        while *new_pots.last().unwrap() == false {
            new_pots.pop();
        }
        pots = new_pots;
        total_offset += started_at;

        i += 1;

        let result: i32 = pots
            .iter()
            .enumerate()
            .filter_map(|(i, &val)| {
                if val {
                    Some(i as i32 + total_offset)
                }
                else {
                    None
                }
            })
            .sum();

        let increase = result - last_result;

        if increase == last_increase {
            pattern_for += 1;
            if pattern_for >= 20 {
                // Found pattern!
                break;
            }
        }
        else {
            last_increase = increase;
            pattern_for = 0;
        }
        last_result = result;
    }

    // A pattern has been found, working that up to the final val
    let result = last_result as i64 + last_increase as i64 * (STEPS - i + 1) as i64;
    println!("{}", result);
}

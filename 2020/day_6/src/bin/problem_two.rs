use std::fs;
use std::collections::HashSet;

fn main() {
    let total: usize = fs::read_to_string("input.txt")
        .expect("Couldn't find input")
        .split("\n\n")
        .map(|x| {
            let mut total: Option<HashSet<char>> = None;
            for line in x.lines() {
                let mut letters = HashSet::new();
                for letter in line.chars() {
                    if letter.is_alphabetic() {
                        letters.insert(letter);
                    }
                }
                if let Some(total_set) = total {
                    total = Some(total_set.intersection(&letters).copied().collect());
                }
                else {
                    total = Some(letters);
                }
            }
            total.unwrap().len()
        })
        .sum();
    
    println!("{}", total);
}

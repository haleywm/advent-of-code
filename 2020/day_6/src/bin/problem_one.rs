use std::fs;
use std::collections::HashSet;

fn main() {
    let total: usize = fs::read_to_string("input.txt")
        .expect("Couldn't find input")
        .split("\n\n")
        .map(|x| {
            let mut letters: HashSet<char> = HashSet::new();
            for letter in x.chars() {
                if letter.is_alphabetic() {
                    letters.insert(letter);
                }
            }
            letters.len()
        })
        .sum();
    
    println!("{}", total);
}

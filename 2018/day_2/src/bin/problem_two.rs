use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    // Reading data into memory because I'll need it all
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();
    
    'top: for (i, a) in lines.iter().enumerate() {
        // Iterating over the lines that come after
        for b in lines.iter().skip(i + 1) {
            let comp = one_off(a, b);
            if let Some(pos) = comp {
                // Found it!
                // Print every other letter in str
                for (i, letter) in a.chars().enumerate() {
                    if i != pos {
                        print!("{}", letter);
                    }
                }
                println!();
                break 'top;
            }
        }
    }

    //println!("{}", result);
}


fn one_off(a: &str, b: &str) -> Option<usize> {
    if a.len() != b.len() {
        return None;
    }
    // Haven't found a dif yet.
    // Set to true when dif found
    // Return false if another dif found
    // Return value if loops ends otherwise
    let mut found_dif = None;
    for (i, (a, b)) in a.chars().zip(b.chars()).enumerate() {
        if a != b {
            if found_dif.is_some() {
                // More than 1 dif
                return None;
            }
            else {
                found_dif = Some(i);
            }
        }
    }
    found_dif
}
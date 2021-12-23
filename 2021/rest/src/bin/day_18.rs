use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Snailfish {
    Num(i64),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

fn main() {
    let file = fs::File::open("input/day_18.txt").expect("Invalid Filename");

    let items: Vec<Snailfish> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            parse_snailfish(&line)
        })
        .collect();

    let sum = items
        .into_iter()
        .reduce(|acc, cur| {
            add_snailfish(acc, cur)
        })
        .unwrap();
    
    println!("{:?}", sum);
}

fn parse_snailfish(input: &str) -> Snailfish {
    // Recursively parse a snailfish string
    // Test if given section is a number
    let num = input.parse::<i64>();
    if let Ok(number) = num {
        Snailfish::Num(number)
    }
    else {
        // Stripping external brackets, panicking if they aren't there
        let input = input.strip_prefix("[").unwrap();
        let input = input.strip_suffix("]").unwrap();
        let first_end = find_end(input);
        let second_end = find_end(&input[(first_end + 1)..]);
        let first_part = &input[..first_end];
        let second_part = &input[(first_end + 1)..(first_end + 1 + second_end)];

        Snailfish::Pair(Box::new(parse_snailfish(first_part)), Box::new(parse_snailfish(second_part)))
    }
}

fn find_end(input: &str) -> usize {
    // Crawls given input to find the end
    // Stops when it finds a comma not inside a bracket, or the end of string
    // Panics if invalid number of brackets
    let mut bracket_depth: i32 = 0;
    let mut end = 0;
    for cur in input.as_bytes() {
        match *cur {
            b'[' => bracket_depth += 1,
            b']' => bracket_depth -= 1,
            b',' => {
                if bracket_depth == 0 {
                    break;
                }
            },
            _ => {}
        }
        end += 1;
    }
    if bracket_depth != 0 {
        panic!("Invalid string: {}", input);
    }
    end
}

fn add_snailfish(a: Snailfish, b: Snailfish) -> Snailfish {
    // Returns the sum of two snailfish
    Snailfish::Pair(Box::new(a), Box::new(b))
    // TODO: Exploding and splitting
}

fn try_explode(cur: &mut Snailfish, level: usize) -> (Option<i64>, Option<i64>) {
    // Tries to explode something if possible recursively
    match cur {
        Snailfish::Num(_) => (None, None),
        Snailfish::Pair(mut a, mut b) => {
            if level >= 4 {
                (None, None)

            }
            else {
                let res = try_explode(&mut a, level + 1);

                (None, None)
            }
        }
    }
}

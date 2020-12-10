use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut adapters: Vec<i64> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    // Adding the first adaptor
    adapters.push(0);
    // Sorting a list for later efficiency
    adapters.sort_unstable();
    // Then adding the final adapter
    adapters.push(adapters[adapters.len() - 1] + 3);
    // So for each item, I want to go up to 3 higher if possible, otherwise less
    let mut diff_array: Vec<i64> = Vec::with_capacity(adapters.len() - 1);
    // Iterating through to see how many possibilities there are from each item
    for i in 0..(adapters.len() - 1) {
        let mut total = 0;
        for offset in 1..(4.min(adapters.len()-i)) {
            if adapters[i + offset] <= adapters[i] + 3 {
                total += 1;
            }
        }
        diff_array.push(total);
    }
    // Lastly, going through in reverse and 'collecting' the total number of possibilties
    for i in (0..diff_array.len()).rev() {
        let mut new_val = 0;
        for j in 1..(diff_array[i] as usize + 1) {
            if i + j >= diff_array.len() {
                new_val += 1;
            }
            else {
                new_val += diff_array[i + j];
            }
        }
        diff_array[i] = new_val;
    }
    let result = diff_array[0];
    println!("{}", result);
}
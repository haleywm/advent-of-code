use std::fs;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Mask {
    X,
    Off,
    On,
}

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    let mask_regex = Regex::new(r"^mask = (.{36})$").unwrap();
    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();


    // Not storing as actual memory since allocating 2^36 * 4 bytes may not be the greatest idea in the world
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut bit_mask = [Mask::Off; 36];

    for line in lines {
        let line = line.unwrap();
        // Line could be either a mask, or a mem
        let mask = mask_regex.captures(&line);
        if let Some(mask_info) = mask {
            // Overwriting mask
            // The mask should be 36 chars long, and so should bit_mask
            for (val, to) in mask_info[1].chars().zip(bit_mask.iter_mut().rev()) {
                *to = match val {
                    'X' => Mask::X,
                    '1' => Mask::On,
                    '0' => Mask::Off,
                    x => panic!("Invalid mask char: {}", x),
                }
            }
        }
        else {
            // Didn't find a mask, looking for mem
            // If this isn't found panic
            let mem_info = mem_regex.captures(&line).unwrap();
            let mut mem_addr = mem_info[1].parse::<u64>().unwrap();
            let mem_value = mem_info[2].parse::<u64>().unwrap();
            // Applying mask
            let mut float_count: Vec<u64> = Vec::new();
            for (i, val) in bit_mask.iter().enumerate() {
                let i = i as u64;
                match val {
                    Mask::X => float_count.push(i),
                    Mask::Off => {},
                    Mask::On => mem_addr |= 1 << i,
                }
            }
            // Wildcard time
            for n in 0..(1 << float_count.len()) {
                let mut next_addr = mem_addr.clone();
                for (i, pos) in float_count.iter().enumerate() {
                    // Getting the i-th bit in n
                    match (n >> i) & 1 {
                        0 => {
                            next_addr &= !(1 << pos);
                        }
                        1 => {
                            next_addr |= 1 << pos;
                        }
                        _ => panic!("I did bitwise wrong"),
                    }
                }
                mem.insert(next_addr, mem_value);
            }
        }
    }
    println!("{}", mem.len());
    let result: u64 = mem.iter().map(|(_, val)| val).sum();
    println!("{}", result);
}

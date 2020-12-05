use day_7::{run_intcode, string_to_intcode, OwnOrRef::*, BUF_SIZE};
use itertools::Itertools;
use std::fs;
use std::io::{self, BufRead};
use std::sync::mpsc;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut line = String::new();
    io::BufReader::new(file)
        .read_line(&mut line)
        .expect("Couldn't read file");
    let intcode = string_to_intcode(line.trim()).unwrap();
    let result = (0..5)
        .permutations(5)
        .map(|x| {
            let mut prev = 0;
            for mode in x {
                prev = run_input(prev, mode, intcode);
            }
            prev
        })
        .max()
        .unwrap();

    println!("{}", result);
}

fn run_input(last_val: i32, mode: i32, line: [i32; BUF_SIZE]) -> i32 {
    let (in_write, in_read) = mpsc::channel();
    let (out_write, out_read) = mpsc::channel();
    in_write.send(mode).unwrap();
    in_write.send(last_val).unwrap();
    run_intcode(line.clone(), Own(in_read), Own(out_write))
        .join()
        .unwrap();
    out_read.recv().unwrap()
}

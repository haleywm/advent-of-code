use day_13::{run_intcode_io, string_to_intcode};
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut line = String::new();
    io::BufReader::new(file)
        .read_line(&mut line)
        .expect("Couldn't read file");

    // Creating a vector that will be resized as needed to store the data
    let mut screen: Vec<Vec<char>> = Vec::new();

    // Spawning intcode operator
    let (_game_in, game_out, _) = run_intcode_io(string_to_intcode(&line.trim()).unwrap());
    while let Ok(x) = game_out.recv() {
        // Data should come in 3's, so reading the othe 2 values in and panicing if they aren't provided
        let x = x as usize;
        let y = game_out.recv().unwrap() as usize;
        let tile = game_out.recv().unwrap() as usize;

        if y >= screen.len() {
            // Extending the lines out as needed
            screen.resize_with(y + 1, Vec::new);
        }
        if x >= screen[y].len() {
            // Extending the characters out on this line as needed
            screen[y].resize(x + 1, ' ');
        }
        screen[y][x] = match tile {
            0 => ' ',
            1 => '█',
            2 => '░',
            3 => '-',
            4 => '⦿',
            x => panic!("Unexpected number: {}", x),
        }
    }
    // Previous structure is there for rendering, no need for this question (hopefully it was worth the effort in the next part)
    let result: i32 = screen.iter()
        .map(|x| -> i32 {
            x.iter().map(|x| {
                if *x == '░' {
                    1
                }
                else {
                    0
                }
            }).sum()
        })
        .sum();
    
    println!("{}", result);
}

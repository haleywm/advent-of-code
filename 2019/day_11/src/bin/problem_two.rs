use day_11::{run_intcode_io, string_to_intcode};
use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;
//use std::sync::mpsc;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut line = String::new();
    io::BufReader::new(file)
        .read_line(&mut line)
        .expect("Couldn't read file");

    // Creating a hashset to store visited arrays in, since there's no real need for an arraya
    let mut painted: HashMap<(i32, i32), bool> = HashMap::new();
    let mut cur_pos = (0, 0);
    let mut cur_face: i8 = 0;

    // Spawning intcode operator
    let (paint_in, paint_out, _) = run_intcode_io(string_to_intcode(&line.trim()).unwrap());
    // Starting off by telling the machine it's on white
    paint_in.send(1).unwrap();
    while let Ok(colour) = paint_out.recv() {
        // Insert the colour, overwriting the old one if needed
        painted.insert(cur_pos, colour == 1);
        // Then, getting the direction
        // Since the machine should send input in groups of 2 if this isn't sent that's an error
        let dir = paint_out.recv().unwrap() == 1;
        // If dir is true, turn right, otherwise left
        cur_face += if dir { 1 } else { -1 };
        // Doing this to make rem work like modulus to handle negative numbers
        cur_face = ((cur_face % 4) + 4) % 4;
        match cur_face {
            // Up
            0 => cur_pos.1 += 1,
            // Right
            1 => cur_pos.0 += 1,
            // Down
            2 => cur_pos.1 -= 1,
            // Left
            3 => cur_pos.0 -= 1,
            // Shouldn't be possible due to modulus
            x => panic!("Invalid direction?? {}", x),
        }
        // Laslty, writing the colour of the new tile
        // If it fails that means that the next pass of the while loop will also fail so ignoring the output
        let _ = paint_in.send(*painted.get(&cur_pos).unwrap_or(&false)as i64);
    }
    // If nothing else is being return program must have finished
    
    // Now to turn this into something displayable
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for ((x, y), _white) in painted.iter() {
        if *x < min_x {
            min_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    }
    let mut result = String::with_capacity(((max_x - min_x + 1) * (max_y - min_y)) as usize);
    for y in (min_y..(max_y + 1)).rev() {
        for x in min_x..(max_x + 1) {
            result.push(
                if *painted.get(&(x, y)).unwrap_or(&false) {
                    '█'
                }
                else {
                    ' '
                }
            )
        }
        result.push('\n');
    }
    print!("{}", result);
}

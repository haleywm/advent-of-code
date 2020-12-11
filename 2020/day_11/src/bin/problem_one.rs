use std::fs;
use std::io::{self, BufRead};
use std::fmt;

#[derive(Clone)]
enum Cell {
    Floor,
    Empty,
    Taken
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Cell::Floor => '.',
            Cell::Empty => 'L',
            Cell::Taken => '#',
        })
    }
}

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    // Creating a 2d array out of the data
    let mut room: Vec<Vec<Cell>> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let mut new = Vec::with_capacity(line.len());

        for byte in line.as_bytes() {
            new.push(
                match byte {
                    b'L' => Cell::Empty,
                    b'#' => Cell::Taken,
                    b'.' => Cell::Floor,
                    x => panic!("Unexpected byte: {}", x),
                }
            )
        }

        room.push(new);
    }
    let mut adjacent = vec![vec![0; room[0].len()]; room.len()];

    // After that's been done, simulating the moment until complete
    // NOTE: With cellular automata, and many other computing things
    // It's very difficult to tell if something will 'halt' (cease movement)
    // Without processing it and seeing if it ends.
    // If a non-halting scenario is given this will loop forever.
    let mut changed = true;
    let max_y = room.len() - 1;
    let max_x = room[0].len() - 1;
    while changed {
        changed = false;
        // First, iterating through and building an adjacency matrix
        for y in 0..=max_y {
            for x in 0..=max_x {
                adjacent[y][x] = count_adjacent(&room, x, y);
            }
        }
        // Then using those values to make changes to that everything happens at once instead of earlier changes influencing later ones
        for y in 0..=max_y {
            for x in 0..=max_x {
                match &room[y][x] {
                    Cell::Empty => {
                        // If a seat is empty and there are no adjacent seats, it becomes occupied
                        if adjacent[y][x] == 0 {
                            room[y][x] = Cell::Taken;
                            changed = true;
                        }
                    }
                    Cell::Taken => {
                        // If a seat is taken and there are 4 or more adjacent seats, it becomes empty
                        if adjacent[y][x] >= 4 {
                            room[y][x] = Cell::Empty;
                            changed = true;
                        }
                    }
                    Cell::Floor => {
                        // Do nothing for floors
                    }
                }
            }
        }
        /*
        println!("Turn:");
        for y in 0..=max_y {
            for x in 0..=max_x {
                print!("{}", room[y][x]);
            }
            println!();
        }
        */
    }

    
    
    // After this step, change has stopped happening. Counting occupied seats
    let mut total = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            match room[y][x] {
                Cell::Taken => total += 1,
                _ => {}
            }
        }
    }
    println!("{}", total);
}

fn count_adjacent(room: &Vec<Vec<Cell>>, x: usize, y: usize) -> u8 {
    let mut total = 0;
    let x = x as i32;
    let y = y as i32;
    for cur_y in (y - 1)..=(y + 1) {
        for cur_x in (x - 1)..=(x + 1) {
            if cur_x >= 0 && cur_x < room[0].len() as i32 && cur_y >= 0 && cur_y < room.len() as i32 && (cur_x != x || cur_y != y) {
                total += match room[cur_y as usize][cur_x as usize] {
                    Cell::Taken => 1,
                    _ => 0,
                }
            }
        }
    }

    total
}

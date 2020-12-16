use day_15::{run_intcode_io, string_to_intcode};
use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
enum Tile {
    Floor,
    Wall,
    Dest,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Tile::Floor => ".",
            Tile::Wall => "#",
            Tile::Dest => "!",
        })
    }
}

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut line = String::new();
    io::BufReader::new(file)
        .read_line(&mut line)
        .expect("Couldn't read file");

    let (bot_in, bot_out, _) = run_intcode_io(string_to_intcode(&line.trim()).unwrap());
    let mut start_x = 0;
    let mut start_y = 0;
    let mut cur_x = 0;
    let mut cur_y = 0;
    // 1: north, 2: sourth, 3: west, 4: east
    let mut cur_dir = 1;

    // Creating map with single floor tile at the center, as that must be the case initially
    let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
    map.insert((0, 0), Tile::Floor);

    // The bot AI will attempt to map the world out by following the "keep going right" strategy
    // Because that should hopefully work
    for _ in 0..10 {
        // Try to go in the current direction, if a wall is hit, turn right, if the spot is found, stop
        bot_in.send(cur_dir).unwrap();
        let feedback = bot_out.recv().unwrap();
        match feedback {
            0 => {
                // Hit wall
                let dir = map_dir(cur_dir);
                let wall_pos = (cur_x + dir.0, cur_y + dir.1);
                map.insert(wall_pos, Tile::Wall);
                cur_dir = change_dir(cur_dir);
            }
            1 => {
                // Floor
                let dir = map_dir(cur_dir);
                cur_x += dir.0;
                cur_y += dir.1;
                map.insert((cur_x, cur_y), Tile::Floor);
            }
            2 => {
                // Found it!
                let dir = map_dir(cur_dir);
                cur_x += dir.0;
                cur_y += dir.1;
                map.insert((cur_x, cur_y), Tile::Dest);
                println!("Found at {}, {}", cur_x, cur_y);
                break;
            }
            x => panic!("Unrecognized output: {}", x),
        }
        draw_map(&map, (cur_x, cur_y));
    }
}

fn map_dir(dir: i64) -> (i32, i32) {
    match dir {
        1 => (0, 1),
        2 => (0, -1),
        3 => (-1, 0),
        4 => (1, 0),
        _ => panic!("No."),
    }
}

fn change_dir(dir: i64) -> i64 {
    match dir {
        1 => 4,
        2 => 3,
        3 => 1,
        4 => 2,
        _ => panic!("No."),
    }
}

fn draw_map(map: &HashMap<(i32, i32), Tile>, pos: (i32, i32)) {
    // Printing current sitation
    print!("---");
    let mut items: Vec<(&(i32, i32), &Tile)> = map.iter().collect();
    let mut min_x = 0;
    let mut prev_y: Option<i32> = None;
    items.sort_by(|(a, _), (b, _)| {
        // Reading info while I'm at it to cheat
        if a.0 < min_x {
            min_x = a.0;
        }
        if b.0 < min_x {
            min_x = b.0;
        }
        // Sort by y first, then x
        let ord = a.1.cmp(&b.1);
        if let Ordering::Equal = ord {
            // They are equal, so compare x
            a.0.cmp(&b.0)
        }
        else {
            ord
        }
    });
    for ((cur_x, cur_y), tile) in items {
        // Checking if new line
        if prev_y.is_none() || prev_y.unwrap() < *cur_y {
            // New line
            print!("\n{}", " ".repeat((cur_x - min_x) as usize));
            prev_y = Some(*cur_y);
        }
        // Printing cur tile
        if *cur_x != pos.0 || *cur_y != pos.1 {
            print!("{}", tile);
        }
        else {
            // We're here
            print!("D");
        }
    }
    // Done
    println!("\n---");
    //println!("{:?}", map);
}
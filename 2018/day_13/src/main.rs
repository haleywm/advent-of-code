use std::fs;
use std::io::{self, BufRead};
use std::mem;

// Defining some stucts and enums to help make things make more sense

#[derive(Clone, PartialEq, Debug)]
enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

use Rotation::*;

impl Rotation {
    pub fn is_vert(&self) -> bool {
        match self {
            Up | Down => true,
            Left | Right => false,
        }
    }

    pub fn move_pos(&self, pos: (usize, usize)) -> (usize, usize) {
        // Y, X coords with 0,0 in top left
        //println!("{:?}", pos);
        match self {
            Up => (pos.0 - 1, pos.1),
            Right => (pos.0, pos.1 + 1),
            Down => (pos.0 + 1, pos.1),
            Left => (pos.0, pos.1 - 1),
        }
    }

    pub fn move_around(&mut self, a: &Rotation, b: &Rotation) {
        // Moves the point around a corner with sides on the two listed rotations
        let forwards = if self.is_vert() {
            *self == a.rev()
        }
        else {
            *self == b.rev()
        };
        if self.is_vert() {
            *self = b.clone();
        }
        else {
            *self = a.clone();
        }
        if !forwards {
            *self = self.rev();
        }
    }

    pub fn add(&mut self, a: &Rotation) {
        let dir_to_num = |dir: &Rotation| {
            match dir {
                Up => 0,
                Right => 1,
                Down => 2,
                Left => 3,
            }
        };
        let total = (dir_to_num(self) + dir_to_num(a)) % 4;
        *self = match total {
            0 => Up,
            1 => Right,
            2 => Down,
            3 => Left,
            _ => panic!("No"),
        }
    }

    fn rev(&self) -> Rotation {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}

#[derive(Debug)]
enum Tile {
    Empty,
    Track(Rotation),
    // Two angles the corner connects, vert then hor
    Corner(Rotation, Rotation),
    Intersection,
    // Current dir, adjustment to make next intersection, the tile below, the last tick parsed
    Cart(Rotation, Rotation, Option<Box<Tile>>, usize),
}
use Tile::*;

fn main() {
    // Not going to have logic to ensure that it's a valid map, just going to parse it and crash if unspecified behaviour happens
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();

    let mut table = Vec::new();
    let mut cart_count = 0;

    let mut width: Option<usize> = None;
    for line in lines {
        let line = line.unwrap();
        // If width is known use it otherwise next time
        let mut row = match width {
            Some(len) => Vec::with_capacity(len),
            None=> Vec::new(),
        };

        for next in line.chars() {
            row.push(
                // Matching every possible char
                match next {
                    ' ' => Empty,
                    '-' => Track(Right),
                    '|' => Track(Up),
                    '/' => Corner(Down, Right),
                    '\\' => Corner(Down, Left),
                    '+' => Intersection,
                    '^' => {
                        cart_count += 1;
                        Cart(Up, Left, Some(Box::new(Track(Up))), 0)
                    },
                    '>' => {
                        cart_count += 1;
                        Cart(Right, Left, Some(Box::new(Track(Right))), 0)
                    },
                    'v' => {
                        cart_count += 1;
                        Cart(Down, Left, Some(Box::new(Track(Up))), 0)
                    },
                    '<' => {
                        cart_count += 1;
                        Cart(Left, Left, Some(Box::new(Track(Right))), 0)
                    },
                    x => panic!("Unrecognized char {}", x),
                }
            );
        }

        // Ensuring square
        if width.is_some() {
            assert_eq!(width.unwrap(), row.len());
        }
        else {
            width = Some(row.len());
        }

        table.push(row);
    }

    // And now perfoming ticks until a crash occurs
    let height = table.len();
    let width = width.unwrap();
    let mut tick = 1;
    let last_cart = 'tick_loop: loop {
        //println!("Tick {}!", tick);
        tick += 1;
        //if tick > 100 {
        //    break (0, 0);
        //}
        let final_cart = cart_count <= 1;
        for row in 0..height {
            for col in 0..width {
                let cur = &mut table[row][col];
                // Do something if this tile is a cart
                if let Cart(dir, _, below, last_tick) = cur {
                    if final_cart {
                        // Final cart! Just return position
                        break 'tick_loop Some((row, col));
                    }
                    if *last_tick == tick {
                        // Already touched this
                        continue;
                    }
                    else {
                        *last_tick = tick;
                    }
                    //println!("{}, {}", row, col);
                    let new_pos = dir.move_pos((row, col));
                    // Taking the value out of the option so that I can safely swap it
                    let below = below.take().unwrap();
                    // Then replacing the prev tile with the contained item in the card, returning the cart
                    let mut cart = mem::replace(&mut table[row][col], *below);
                    // Now I just need to get the new tile, perform rotation logic if needed, put it in carts (now empty) box, and put cart in that pos
                    // This var is used if I'm moving onto a cart, so I should remove both
                    let mut remove_carts = false;
                    if let Cart(dir, dir_intr, below, _) = &mut cart {
                        // Seeing if the new tile needs special action
                        //println!("{:?}", table[new_pos.0][new_pos.1]);
                        match &table[new_pos.0][new_pos.1] {
                            Corner(dir_one, dir_two) => {
                                // Rotate around corner
                                dir.move_around(dir_one, dir_two)
                            }
                            Intersection => {
                                // Rotate as needed for intersection
                                dir.add(dir_intr);
                                *dir_intr = match dir_intr {
                                    Left => Up,
                                    Up => Right,
                                    Right => Left,
                                    Down => Up,
                                };
                            }
                            Cart(..) => {
                                // Rather than break, remove both carts and print the position
                                println!("{}, {}", new_pos.1, new_pos.0);
                                remove_carts = true;
                            }
                            // Otherwise don't care
                            _ => {}
                        }
                        //println!("{:?}", dir);
                        // Temporatily making the entry Empty while shifting the below value into the card
                        *below = Some(Box::new(mem::replace(&mut table[new_pos.0][new_pos.1], Empty)));
                    }
                    else {
                        panic!("I did memory bad");
                    }
                    if remove_carts {
                        // I'm moving onto a cart, so I should make the upcoming point equal to the doubly-nested below val
                        if let Cart(.., below, _) = cart {
                            if let Cart(.., below, _) = *below.unwrap() {
                                table[new_pos.0][new_pos.1] = *below.unwrap();
                                cart_count -= 2;
                            }
                            else {
                                panic!("No");
                            }
                        }
                        else {
                            panic!("No");
                        }
                    }
                    else {
                        // Business as usual
                        table[new_pos.0][new_pos.1] = cart;
                    }
                }
            }
        }
        if final_cart {
            // Reached end of loop with no cart
            break None;
        }
    };
    match last_cart {
        Some((y, x)) => println!("Last Cart at: {}, {}", x, y),
        None => println!("No carts remain"),
    }
}

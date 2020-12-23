use std::fs;
use std::io::{self, BufRead};
use std::cell::RefCell;

const TILE_SIZE: usize = 10;

// Tile number, pos (rotation, flipped, x, y), value
//type Tile = (u64, Option<(usize, usize, i32, i32)>, [[bool; TILE_SIZE]; TILE_SIZE]);

struct Tile {
    num: u64,
    pos: RefCell<Option<TilePos>>,
    val: [[bool; TILE_SIZE]; TILE_SIZE],
}

#[derive(Debug)]
struct TilePos {
    rotation: usize,
    flip_vert: bool,
    flip_hor: bool,
    x: i32,
    y: i32,
}

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file)
        .lines();
    
    let mut tiles: Vec<Tile> = Vec::new();
    
    while let Some(Ok(tile_info)) = lines.next() {
        // The first line of tile info gives the number of the tile
        // Format: Tile {num}:, so take th
        let tile_num = tile_info[5..tile_info.len() - 1].parse().unwrap();
        let mut tile_contents = [[false; TILE_SIZE]; TILE_SIZE];
        for i in 0..TILE_SIZE {
            let next_line = lines.next().unwrap().unwrap();
            assert_eq!(TILE_SIZE, next_line.len());
            for (j, item) in next_line.as_bytes().iter().enumerate() {
                tile_contents[i][j] = *item == b'#';
            }
        }
        // Insert the created tile
        tiles.push(Tile { num: tile_num, pos: RefCell::new(None), val: tile_contents});
        // Get rid of the next line if there is one as it will be blank (last item doesn't have one)
        lines.next();
    }

    let image_size = (tiles.len() as f32).sqrt() as usize;
    assert_eq!(image_size * image_size, tiles.len());
    println!("{} x {}: {}", image_size, image_size, tiles.len());

    // Starting with an arbitrary piece declared to be 0, 0, with it's original rotation and flip 0, find matching pieces from there
    // Pieces can match to 1 of 4 sides, and have 1 of 4 rotations. In addition, they can be flipped either horizontally, vertically, or neither
    // (A flip in both directions is equivalent to a double rotation)
    *tiles[0].pos.borrow_mut() = Some(TilePos {rotation: 0, flip_vert: false, flip_hor: false, x: 0, y: 0});

    let mut to_eval = vec![0];
    
    while !to_eval.is_empty() {
        let cur_idx = to_eval.pop().unwrap();
        let cur_val = &tiles[cur_idx];
        'tile_test: for comp_idx in 0..tiles.len() {
            if tiles[comp_idx].pos.borrow().is_some() {
                // This has already been set, don't bother
                continue;
            }
            let comp_val = &tiles[comp_idx];
            for side_from in 0..4 {
                for side_to in 0..4 {
                    'side_test: for forwards in &[true, false] {
                        for i in 0..TILE_SIZE {
                            let pos_from = convert_pos(side_from, i, true);
                            let pos_to = convert_pos(side_to, i, *forwards);

                            if cur_val.val[pos_from.0][pos_from.1] != comp_val.val[pos_to.0][pos_to.1] {
                                // Doesn't match
                                continue 'side_test;
                            }
                        }
                        // Found a match!
                        let cur_pos = cur_val.pos.borrow();
                        let cur_pos = cur_pos.as_ref().unwrap();
                        // Add the sides being compared from and to and subtract 2 to get relative rotation (i.e. a comp from side 1 to side 3 would match with 0 rot)
                        // Then add it to parent rotation then modulus
                        let mut rotation = ((((cur_pos.rotation as i32 + side_from as i32 - side_to as i32 - 2) % 4) + 4) % 4) as usize;
                        
                        let mut flip_vert = ((!forwards) && (side_from == 1 || side_from == 3)) ^ cur_pos.flip_vert;
                        let mut flip_hor = ((!forwards) && (side_from == 0 || side_from == 2)) ^ cur_pos.flip_hor;
                        if flip_vert && flip_hor {
                            // Flipped both ways, just rotate 180 and call it a day
                            flip_vert = false;
                            flip_hor = false;
                            rotation = (rotation + 2) % 4;
                        }

                        // Lastly, getting the change in position
                        // This part isn't too complex, just get the side the parent comes from, and apply it's flip if needed
                        let side_from_adj = (side_from + cur_pos.rotation) % 4;
                        let (x, y) = if (side_from_adj == 0 && !cur_pos.flip_vert) || (side_from_adj == 2 && cur_pos.flip_vert) {
                            (cur_pos.x, cur_pos.y + 1)
                        }
                        else if (side_from_adj == 1 && !cur_pos.flip_hor) || (side_from_adj == 3 && cur_pos.flip_hor) {
                            (cur_pos.x + 1, cur_pos.y)
                        }
                        else if (side_from_adj == 2 && !cur_pos.flip_vert) || (side_from_adj == 0 && cur_pos.flip_vert) {
                            (cur_pos.x, cur_pos.y - 1)
                        }
                        else if (side_from_adj == 3 && !cur_pos.flip_hor) || (side_from_adj == 1 && cur_pos.flip_hor) {
                            (cur_pos.x - 1, cur_pos.y)
                        }
                        else {
                            panic!("Oops")
                        };
                        *comp_val.pos.borrow_mut() = Some(TilePos {rotation, flip_vert, flip_hor, x, y});
                        println!("From {}: {}: ({}:{}) {:?}", cur_val.num, comp_val.num, side_from, side_to, comp_val.pos.borrow());
                        to_eval.push(comp_idx);
                        continue 'tile_test;
                    }
                }
            }
        }
    }
}

fn convert_pos(dir: usize, idx: usize, forwards: bool) -> (usize, usize) {
    // Converts a direction to the vec coordinates
    let idx = if forwards {
        idx
    }
    else {
        TILE_SIZE - 1 - idx
    };
    match dir {
        // Top
        0 => (0, idx),
        // Right
        1 => (idx, TILE_SIZE - 1),
        // Bottom
        2 => (TILE_SIZE - 1, idx),
        // Left
        3 => (idx, 0),
        _ => panic!("No"),
    }
}

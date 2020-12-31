use std::fs;
use std::io::{self, BufRead};
use std::cell::RefCell;

const TILE_SIZE: usize = 10;

// Tile number, pos (rotation, flipped, x, y), value
//type Tile = (u64, Option<(usize, usize, i32, i32)>, [[bool; TILE_SIZE]; TILE_SIZE]);

// ALlowing dead code because num is important in some versions
#[allow(dead_code)]
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
        for y in 0..TILE_SIZE {
            let next_line = lines.next().unwrap().unwrap();
            assert_eq!(TILE_SIZE, next_line.len());
            for (x, item) in next_line.as_bytes().iter().enumerate() {
                tile_contents[x][y] = *item == b'#';
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

    let mut min_x = 0;
    let mut min_y = 0;

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
                        let rel_roation = ((((side_from as i32 - side_to as i32 - 2) % 4) + 4) % 4) as usize;
                        let mut rotation = (cur_pos.rotation + rel_roation) % 4;
                        
                        // If relative rotation is 2, the reverse is because they're facing opposite directions
                        let mut flip_vert = (((!forwards) ^ (rel_roation == 2)) && (side_from == 1 || side_from == 3)) ^ cur_pos.flip_vert;
                        let mut flip_hor = (((!forwards) ^ (rel_roation == 2)) && (side_from == 0 || side_from == 2)) ^ cur_pos.flip_hor;
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
                            (cur_pos.x, cur_pos.y - 1)
                        }
                        else if (side_from_adj == 1 && !cur_pos.flip_hor) || (side_from_adj == 3 && cur_pos.flip_hor) {
                            (cur_pos.x + 1, cur_pos.y)
                        }
                        else if (side_from_adj == 2 && !cur_pos.flip_vert) || (side_from_adj == 0 && cur_pos.flip_vert) {
                            (cur_pos.x, cur_pos.y + 1)
                        }
                        else if (side_from_adj == 3 && !cur_pos.flip_hor) || (side_from_adj == 1 && cur_pos.flip_hor) {
                            (cur_pos.x - 1, cur_pos.y)
                        }
                        else {
                            panic!("Oops")
                        };

                        if x < min_x {
                            min_x = x;
                        }
                        if y < min_y {
                            min_y = y;
                        }

                        *comp_val.pos.borrow_mut() = Some(TilePos {rotation, flip_vert, flip_hor, x, y});
                        println!("From {}: {}: ({}:{}) {:?}", cur_val.num, comp_val.num, side_from, side_to, comp_val.pos.borrow());
                        to_eval.push(comp_idx);
                        continue 'tile_test;
                    }
                }
            }
        }
    }

    // Now that that mess is out the way, making a single vector of everything with the edge pieces removed
    let tile_res = TILE_SIZE - 2;
    let img_res = image_size * tile_res;
    let mut img_vec = vec![vec![false; img_res]; img_res];

    for tile in tiles.into_iter() {
        let pos = tile.pos.borrow();
        let pos = pos.as_ref().unwrap();
        let x_offset = (pos.x - min_x) as usize * tile_res;
        let y_offset = (pos.y - min_y) as usize * tile_res;
        // Leaving out the edges
        for x in 1..=tile_res {
            for y in 1..=tile_res {
                // Rotating as needed
                let (to_x, to_y) = translate_pos(x - 1, y - 1, pos.rotation, tile_res, tile_res, pos.flip_hor, pos.flip_vert);

                let to_x = x_offset + to_x;
                let to_y = y_offset + to_y;

                img_vec[to_x][to_y] = tile.val[x][y];
            }
        }
        /*
        println!("{}:", tile.num);

        for y in y_offset..y_offset+tile_res {
            for x in x_offset..x_offset+tile_res {
                print!("{}", if img_vec[x][y] { '#' } else { '.' });
            }
            println!();
        }
        */
    }
    // Print
    /*
    for y in 0..img_res {
        for x in 0..img_res {
            print!("{}", if img_vec[x][y] { '#' } else { '.' });
        }
        println!();
    }
    */

    // Finally, looking for sea monsters
    let monster_shape = vec![
        (0, 1),
        (1, 2),
        (4, 2),
        (5, 1),
        (6, 1),
        (7, 2),
        (10, 2),
        (11, 1),
        (12, 1),
        (13, 2),
        (16, 2),
        (17, 1),
        (18, 0),
        (18, 1),
        (19, 1),
    ];
    let shape_width = 20;
    let shape_height = 3;

    let mut orientation: Option<TilePos> = None;

    // First scanning in any direction until I find the correct orientation
    'or_check: for y in 0..img_res {
        for x in 0..img_res {
            // Trying every possible transformation until the right one is found
            for rot in 0..4 {
                'next_pos: for (flip_hor, flip_vert) in vec![(false, false), (true, false), (false, true)] {
                    for point in monster_shape.iter() {
                        let (check_x, check_y) = translate_pos(point.0, point.1, rot, shape_width, shape_height, flip_hor, flip_vert);
                        let check_x = check_x + x;
                        let check_y = check_y + y;
                        if check_x < img_res && check_y < img_res {
                            // Fits within shape
                            if !img_vec[check_x][check_y] {
                                // Doesn't fit
                                continue 'next_pos;
                            }
                        }
                        else {
                            continue 'next_pos;
                        }
                    }
                    // Found a monster!
                    orientation = Some(TilePos { rotation: rot, flip_vert, flip_hor, x: 0, y: 0 });
                    break 'or_check;
                }
            }
        }
    }
    let mut monst_count = 0;
    let orientation = orientation.expect("Couldn't find any dragons on any orientation");
    for y in 0..img_res-shape_height+1 {
        'next_seek_pos: for x in 0..img_res-shape_height+1 {
            for point in monster_shape.iter() {
                let (check_x, check_y) = translate_pos(point.0, point.1, orientation.rotation, shape_width, shape_height, orientation.flip_hor, orientation.flip_vert);
                let check_x = check_x + x;
                let check_y = check_y + y;
                if check_x < img_vec.len() && check_y < img_vec[0].len() {
                    // Fits within shape
                    if !img_vec[check_x][check_y] {
                        // Doesn't fit
                        continue 'next_seek_pos;
                    }
                }
                else {
                    continue 'next_seek_pos;
                }
            }
            // Found a monster!
            monst_count += 1;
        }
    }
    let result: usize = img_vec.iter()
        .map(|x| x.iter().filter(|y| **y).count())
        .sum::<usize>() - monst_count * 15;
    println!("{}", result);
}

fn translate_pos(x: usize, y: usize, rot: usize, width: usize, height: usize, flip_hor: bool, flip_vert: bool) -> (usize, usize) {
    // Rotating as needed
    let (to_x, to_y) = match rot {
        0 => (x, y),
        1 => (height - 1 - y, x),
        2 => (width - 1 - x, height - 1 - y),
        3 => (y, width - 1 - x),
         _ => panic!("No"),
    };
    // Then lastly transforming by flip
    let to_x = if !flip_hor {
        to_x
    }
    else {
        if rot == 0 || rot == 2 {
            width - 1 - to_x
        }
        else {
            // Using height's boundries
            height - 1 - to_x
        }
    };
    let to_y = if !flip_vert {
        to_y
    }
    else {
        if rot == 0 || rot == 2 {
            height - 1 - to_y
        }
        else {
            // Using width's boundries
            width - 1 - to_y
        }
    };
    (to_x, to_y)
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
        0 => (idx, 0),
        // Right
        1 => (TILE_SIZE - 1, idx),
        // Bottom
        2 => (idx, TILE_SIZE - 1),
        // Left
        3 => (0, idx),
        _ => panic!("No"),
    }
}

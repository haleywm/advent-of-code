use std::fs;
use std::io::{self, BufRead};

const TILE_SIZE: usize = 10;

// Tile number, pos (rotation, flipped, x, y), value
type Tile = (u64, Option<(usize, usize, i32, i32)>, [[bool; TILE_SIZE]; TILE_SIZE]);

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
        tiles.push((tile_num, None, tile_contents));
        // Get rid of the next line if there is one as it will be blank (last item doesn't have one)
        lines.next();
    }

    let image_size = (tiles.len() as f32).sqrt() as usize;
    assert_eq!(image_size * image_size, tiles.len());
    println!("{} x {}: {}", image_size, image_size, tiles.len());


    // Start with the first tile at 0, 0 facing upwards, then attach tiles until a shape is formed
    tiles[0].1 = Some((0, 0, 0, 0));
    let mut to_visit = vec![0];

    while !to_visit.is_empty() {
        let index = to_visit.pop().unwrap();
        let cur_tile = tiles[index];
        let side_conv = |x: usize, i: usize| {
            match x {
                // Top
                0 => (0, i),
                // Right
                1 => (i, TILE_SIZE - 1),
                // Bottom
                2 => (TILE_SIZE - 1, i),
                // Left
                3 => (i, 0),
                _ => panic!("No"),
            }
        };
        // Filter may need adjustment if there are issues with overlapping image sides, as this only looks at what is needed
        'each_tile: for comp_index in 0..tiles.len() {
            let comp_tile = &tiles[comp_index];
            if comp_tile.1.is_some() {
                // This one is already known
                continue 'each_tile;
            }
            for side_1 in 0..4 {
                for side_2 in 0..4 {
                    'side_comp: for forward in &[true, false] {
                        for point in 0..TILE_SIZE {
                            let pos_1 = side_conv(side_1, point);
                            
                            let pos_2 = if *forward {
                                side_conv(side_2, point)
                            } else {
                                side_conv(side_2, TILE_SIZE - 1 - point)
                            };
                            
                            if cur_tile.2[pos_1.0][pos_1.1] != comp_tile.2[pos_2.0][pos_2.1] {
                                // This one doesn't match, move on to next comparison
                                continue 'side_comp;
                            }
                        }
                        // Made it here, that means this side matches

                        // If forward is false, this tile being compared is flipped relative to the current tile
                        // Flipping has 3 possible values
                        // 0: No flip
                        // 1: Horizontal flip
                        // 2: Vertical flip
                        // 3: Both
                        let rel_flipped = cur_tile.1.unwrap().1 ^
                            if *forward {
                                0
                            }
                            else if side_1 + cur_tile.1.unwrap().0 == 0 || side_1 + cur_tile.1.unwrap().0 == 2 {
                                1
                            }
                            else {
                                2
                            };

                        let pos = match
                            (side_1 + cur_tile.1.unwrap().0) % 4
                        {
                            // The side the new one goes on
                            // Top
                            0 => (cur_tile.1.unwrap().2, cur_tile.1.unwrap().3 + 
                                if rel_flipped >> 1 & 1 == 0 {
                                    1
                                }
                                else {
                                    -1
                                }
                            ),
                            // Right
                            1 => (cur_tile.1.unwrap().2 + 
                                if rel_flipped & 1 == 0 {
                                    1
                                }
                                else {
                                    -1
                                }
                            , cur_tile.1.unwrap().3),
                            // Bottom
                            2 => (cur_tile.1.unwrap().2, cur_tile.1.unwrap().3 - 
                                if rel_flipped >> 1 & 1 == 0 {
                                    1
                                }
                                else {
                                    -1
                                }
                            ),
                            // Left
                            3 => (cur_tile.1.unwrap().2 - 
                                if rel_flipped & 1 == 0 {
                                    1
                                }
                                else {
                                    -1
                                }
                            , cur_tile.1.unwrap().3),
                            _ => panic!("No"),
                        };
                        // Getting the pos of side_2 relative to side_1, sub 2, add it to the current rotation, and then mod 4
                        // Maf
                        let rotation = ((((cur_tile.1.unwrap().0 as i32 + side_2 as i32 - side_1 as i32 - 2) % 4) + 4) % 4) as usize;

                        println!("From {}: {}: {}, {}, ({}, {})", cur_tile.0, comp_tile.0, rotation, rel_flipped, pos.0, pos.1);

                        tiles[comp_index].1 = Some((rotation, rel_flipped, pos.0, pos.1));
                        to_visit.push(comp_index);
                        continue 'each_tile;
                    }
                }
            }
        }
    }
    // for tile in tiles {
    //     if let Some(pos) = tile.1 {
    //         println!("{}: {}, ({}, {})", tile.0, pos.0, pos.1, pos.2);
    //     }
    //     else {
    //         println!("{}: None", tile.0);
    //     }
    // }

    // let mut total = 1;
    // for i in 0..tiles.len() {
    //     let cur_tile = tiles[i];
    //     let mut cur_matches = 0;
    //     // There should be 4 tiles with only 2 sides that match the sides of other tiles
    //     for j in i..tiles.len() {
    //         let comp_tile = tiles[j];
    //         let side_conv = |x: usize, i: usize| {
    //             match x {
    //                 // Right
    //                 0 => (0, i),
    //                 // Left
    //                 1 => (TILE_SIZE - 1, i),
    //                 // Top
    //                 2 => (i, 0),
    //                 // Bottom
    //                 3 => (i, TILE_SIZE - 1),
    //                 _ => panic!("No"),
    //             }
    //         };
    //         for side_1 in 0..4 {
    //             for side_2 in 0..4 {
    //                 'side_comp: for forward in &[true, false] {
    //                     for point in 0..TILE_SIZE {
    //                         let pos_1 = side_conv(side_1, point);
                            
    //                         let pos_2 = if *forward {
    //                             side_conv(side_2, point)
    //                         } else {
    //                             side_conv(side_2, TILE_SIZE - 1 - point)
    //                         };
                            
    //                         if cur_tile.2[pos_1.0][pos_1.1] != comp_tile.2[pos_2.0][pos_2.1] {
    //                             // This one doesn't match, move on to next comparison
    //                             continue 'side_comp;
    //                         }
    //                     }
    //                     // Made it here, that means this side matches
    //                     cur_matches += 1;
    //                 }
    //             }
    //         }
    //     }
    //     // Now that the number of known sides has been found, if it's 2 then that's a corner tile
    //     if cur_matches == 2 {
    //         println!("{}", cur_tile.0);
    //         total *= cur_tile.0;
    //     }
    // }
    // println!("{}", total);
}

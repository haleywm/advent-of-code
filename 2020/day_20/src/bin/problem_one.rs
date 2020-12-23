use std::fs;
use std::io::{self, BufRead};

const TILE_SIZE: usize = 10;

type Tile = (u64, [[bool; TILE_SIZE]; TILE_SIZE]);

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
        tiles.push((tile_num, tile_contents));
        // Get rid of the next line if there is one as it will be blank (last item doesn't have one)
        lines.next();
    }

    let image_size = (tiles.len() as f32).sqrt() as usize;
    assert_eq!(image_size * image_size, tiles.len());
    println!("{} x {}: {}", image_size, image_size, tiles.len());
    let mut total = 1;
    for i in 0..tiles.len() {
        let cur_tile = tiles[i];
        let mut cur_matches = 0;
        // There should be 4 tiles with only 2 sides that match the sides of other tiles
        for j in (0..tiles.len()).filter(|j| *j != i) {
            let comp_tile = tiles[j];
            let side_conv = |x: usize, i: usize| {
                match x {
                    0 => (0, i),
                    1 => (TILE_SIZE - 1, i),
                    2 => (i, 0),
                    3 => (i, TILE_SIZE - 1),
                    _ => panic!("No"),
                }
            };
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
                            
                            if cur_tile.1[pos_1.0][pos_1.1] != comp_tile.1[pos_2.0][pos_2.1] {
                                // This one doesn't match, move on to next comparison
                                continue 'side_comp;
                            }
                        }
                        // Made it here, that means this side matches
                        cur_matches += 1;
                    }
                }
            }
        }
        // Now that the number of known sides has been found, if it's 2 then that's a corner tile
        if cur_matches == 2 {
            println!("{}", cur_tile.0);
            total *= cur_tile.0;
        }
    }
    println!("{}", total);
}

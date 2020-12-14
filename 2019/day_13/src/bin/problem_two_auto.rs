use day_13::{run_intcode_io_synced, string_to_intcode};
use std::fs;
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut line = String::new();
    io::BufReader::new(file)
        .read_line(&mut line)
        .expect("Couldn't read file");
    let mut intcode = string_to_intcode(&line.trim()).unwrap();
    intcode[0] = 2;

    // Creating a vector that will be resized as needed to store the data
    let mut screen: Vec<Vec<char>> = Vec::new();
    let mut score = 0;
    // Spawning intcode operator
    let (game_in, game_out, _) = run_intcode_io_synced(intcode);

    // Stores current values of flipper and ball for control decisions
    let mut ball_x: Option<i64> = None;
    let mut flipper_x: Option<i64> = None;
    let mut tile_count = 0;
    let mut end = false;
    'game_loop: loop {
        if end {
            break 'game_loop;
        }
        // Only send input if both ball and flipper are known
        let dir = get_dir(ball_x, flipper_x);
        let send = match dir {
            Some(dir) => Some(game_in.try_send(dir)),
            None => None
        };
        match send {
            None | Some(Ok(_)) | Some(Err(mpsc::TrySendError::Full(_))) => {
                // Input has been recieved, or it wasn't looking for input at the moment
                // To ensure that all input has been processed, repeat until nothing
                while let Ok(x) = game_out.recv_timeout(Duration::from_nanos(500)) {
                //while let Ok(x) = game_out.try_recv() {
                    // Data should come in 3's, so reading the othe 2 values in and panicing if they aren't provided
                    if x < 0 {
                        // This is the score, ignoring the y value (could test to ensure it's 0 and panic if not but eh)
                        let _ = game_out.recv().unwrap();
                        score = game_out.recv().unwrap();
                    } else {
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
                        if screen[y][x] == '░' && tile != 2 {
                            // A tile is being removed
                            tile_count -= 1;
                            // If the last tile is removed, end the game
                            if tile_count == 0 {
                                end = true;
                            }
                        }
                        screen[y][x] = match tile {
                            0 => ' ',
                            1 => '█',
                            2 => {
                                if screen[y][x] != '░' {
                                    // A new tile is being added, incease count
                                    tile_count += 1;
                                }
                                '░'
                            }
                            3 => {
                                flipper_x = Some(x as i64);
                                '-'
                            }
                            4 => {
                                ball_x = Some(x as i64);
                                ''
                            }
                            x => panic!("Unexpected number: {}", x),
                        };
                        //println!("{}", tile);
                    }
                }
            }
            Some(Err(e)) => {
                // Only other possible error type is dropped channel, meaning it's ended
                panic!("Game ended unexpectedly ({}, {:?}, {:?}, {:?})", score, ball_x, flipper_x, e);
            }
        }
    }
    println!("{}", score);
}

fn get_dir(ball_x: Option<i64>, flipper_x: Option<i64>) -> Option<i64> {
    if ball_x.is_none() || flipper_x.is_none() {
        None
    }
    else {
        let ball_x = ball_x.unwrap();
        let flipper_x = flipper_x.unwrap();
        if ball_x > flipper_x {
            // Move right
            Some(1)
        }
        else if ball_x < flipper_x {
            // Move left
            Some(-1)
        }
        else {
            // Equal
            Some(0)
        }
    }
}
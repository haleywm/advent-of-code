use day_13::{run_intcode_io, string_to_intcode};
use std::fs;
use std::io::{self, BufRead};
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
    // This is such a hacky way to do it but anything more complex would need additional communication protocols and uhh
    let time_limit = Duration::from_millis(1);
    // Spawning intcode operator
    let (game_in, game_out, _) = run_intcode_io(intcode);
    loop {
        while let Ok(x) = game_out.recv_timeout(time_limit) {
            // Data should come in 3's, so reading the othe 2 values in and panicing if they aren't provided
            if x < 0 {
                // This is the score, ignroing the y value (could test to ensure it's 0 and panic if not but eh)
                let _ = game_out.recv_timeout(time_limit).unwrap();
                score = game_out.recv_timeout(time_limit).unwrap();
            }
            else {
                let x = x as usize;
                let y = game_out.recv_timeout(time_limit).unwrap() as usize;
                let tile = game_out.recv_timeout(time_limit).unwrap() as usize;

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
                    4 => '',
                    x => panic!("Unexpected number: {}", x),
                }
            }
        }
        // Iterating through the string, getting both the x coord of the ball, the flipper, and the number of walls
        let (ball_x, flipper_x, wall_count) = screen.iter()
            .flat_map(|x| {
                x.iter().enumerate().map(|(pos, letter)| {
                    let mut wall_count = 0;
                    let mut ball_x = None;
                    let mut flipper_x = None;
                    match letter {
                        '░' => wall_count += 1,
                        '-' => flipper_x = Some(pos),
                        '' => ball_x = Some(pos),
                        _ => {}
                    }
                    (ball_x, flipper_x, wall_count)
                })
            })
            .fold((None, None, 0), |(old_ball_x, old_flipper_x, old_wall_count), (ball_x, flipper_x, wall_count)| {
                let ball_x = match ball_x {
                    Some(pos) => Some(pos),
                    None => old_ball_x,
                };
                let flipper_x = match flipper_x {
                    Some(pos) => Some(pos),
                    None => old_flipper_x,
                };
                (ball_x, flipper_x, old_wall_count + wall_count)
            });
        let ball_x = ball_x.unwrap();
        let flipper_x = flipper_x.unwrap();
        // Now that I've collected everything, now to do logic
        if wall_count == 0 {
            // Done
            break;
        }
        if flipper_x < ball_x {
            // Need to move right
            game_in.send(1).unwrap();
        }
        else if flipper_x > ball_x {
            // Need to move left
            game_in.send(-1).unwrap();
        }
        else {
            // They're equal should be fine
            game_in.send(0).unwrap();
        }
    }
    println!("{}", score);
}

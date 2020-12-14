use day_13::{run_intcode_io, string_to_intcode};
use std::fs;
use std::io::{self, stdin, stdout, BufRead, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut line = String::new();
    // Have to do this due to multiple read_line defs
    BufRead::read_line(&mut io::BufReader::new(file), &mut line).expect("Couldn't read file");

    let mut intcode = string_to_intcode(&line.trim()).unwrap();
    intcode[0] = 2;

    // Initializing raw mode
    let stdout = Arc::new(Mutex::new(stdout().into_raw_mode().unwrap()));
    // Clearing the screen
    writeln!(stdout.lock().unwrap(), "{}", clear::All).unwrap();

    let score = Arc::new(Mutex::new(0));
    // Spawning intcode operator
    let (game_in, game_out, _) = run_intcode_io(intcode);
    let score_ref = Arc::clone(&score);
    let stdout_ref = Arc::clone(&stdout);
    thread::spawn(move || {
        while let Ok(x) = game_out.recv() {
            // Data should come in 3's, so reading the othe 2 values in and panicing if they aren't provided
            let y = game_out.recv().unwrap();
            let tile = game_out.recv().unwrap();

            if x >= 0 {
                let tile = match tile {
                    0 => ' ',
                    1 => '█',
                    2 => '░',
                    3 => '-',
                    4 => '',
                    x => panic!("Unexpected number: {}", x),
                };
                writeln!(
                    stdout_ref.lock().unwrap(),
                    "{}{}",
                    cursor::Goto((x + 1) as u16, (y + 1) as u16),
                    tile
                )
                .unwrap();
            } else {
                if x == -1 {
                    let mut score = score_ref.lock().unwrap();
                    *score = tile;
                    writeln!(
                        stdout_ref.lock().unwrap(),
                        "{}{}",
                        cursor::Goto(50, 2),
                        score
                    )
                    .unwrap();
                }
            }
            //io::stdout().flush().unwrap();
        }
    });

    let stdin = stdin();
    for c in stdin.keys() {
        // Getting currently held keys
        let next = match c.unwrap() {
            Key::Left => -1,
            Key::Right => 1,
            //Key::Char('q') => break,
            _ => 0,
        };
        match game_in.send(next) {
            Ok(_) => {}
            Err(_) => break,
        }
    }

    let score = score.lock().unwrap();
    write!(stdout.lock().unwrap(), "{}", score).unwrap();
}

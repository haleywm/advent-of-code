use std::fs;
use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file)
        .lines();
    
    let mut decks: Vec<VecDeque<u64>> = vec![VecDeque::new(); 2];
    let mut player = 0;
    // First, reading player 1's cards, then player 2's
    // Discarding the first line
    lines.next();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.len() == 0 {
            // Found a blank line, must have reached end of first card wrap
            player += 1;
            // Skipping next line
            lines.next();
            // In case there's more blank lines at the end of input break
            if player > 1 {
                break;
            }
        }
        else {
            let card = line.parse().unwrap();
            decks[player].push_back(card);
        }
    }
    // Next, keep comparing until a player has lost
    while decks[0].len() != 0 && decks[1].len() != 0 {
        let card_one = decks[0].pop_front().unwrap();
        let card_two = decks[1].pop_front().unwrap();
        let winner = if card_one > card_two {
            // Player one wins
            0
        }
        else {
            1
        };
        if card_one > card_two {
            decks[winner].push_back(card_one);
            decks[winner].push_back(card_two);
        }
        else {
            decks[winner].push_back(card_two);
            decks[winner].push_back(card_one);
        }
    }

    // Lasty, determining a winner
    let winner = if decks[0].len() != 0 {
        0
    }
    else {
        1
    };
    // And getting score
    let result: u64 = decks[winner]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| {
            (i as u64 + 1) * card
        })
        .sum();
    
    println!("{}", winner + 1);
    println!("{}", result);
}

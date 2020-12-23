use std::fs;
use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file)
        .lines();
    
    let mut decks: Vec<VecDeque<usize>> = vec![VecDeque::new(); 2];
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

    // Lasty, determining a winner
    let (winner, deck) = recursive_game(decks);
    // And getting score
    let result: usize = deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| {
            (i as usize + 1) * card
        })
        .sum();
    
    println!("{}", winner + 1);
    println!("{}", result);
}

fn recursive_game(mut decks: Vec<VecDeque<usize>>) -> (usize, VecDeque<usize>) {
    // Performs a recursive game using the two provided decks, returning 0 or 1 depending on the winner, as well as the winners deck
    // Next, keep comparing until a player has lost

    // Keeps track of the hands in previous rounds
    let mut previous_rounds: Vec<[VecDeque<usize>; 2]> = Vec::new();
    while decks[0].len() != 0 && decks[1].len() != 0 {
        if previous_rounds.iter().any(|val| {
            val[0] == decks[0] && val[1] == decks[1]
        }) {
            // Found a match
            //println!("Loop found, player 1 wins by default");
            return (0, decks.remove(0));
        }
        else {
            // No match, copy current decks to memory
            previous_rounds.push([decks[0].clone(), decks[1].clone()]);
        }
        let cards = [decks[0].pop_front().unwrap(), decks[1].pop_front().unwrap()];
        // If possible, recurse a game
        let winner = 
            if cards[0] <= decks[0].len() && cards[1] <= decks[1].len() {
                // Creating copied sub decks using the remaining cards to perform a battle using
                let mut subdecks = vec![VecDeque::with_capacity(cards[0] + cards[1]); 2];
                for i in 0..2 {
                    // Get the first x items in the deck, equal to the number on the card
                    for num in decks[i].iter().take(cards[i]) {
                        subdecks[i].push_back(*num);
                    }
                }
                // We don't care about the winners deck in sub-recursions
                let res = recursive_game(subdecks).0;
                res
            }
            else {
                // Otherwise select the largest winner
                if cards[0] > cards[1] {
                    // Player one wins
                    0
                }
                else {
                    1
                }
            };
        // Winner is either 1 or 0
        decks[winner].push_back(cards[winner]);
        decks[winner].push_back(cards[1 - winner]);
    }

    // Lasty, determining a winner (whoever doesn't have an empty deck)
    let winner = if decks[0].len() != 0 {
        0
    }
    else {
        1
    };
    (winner, decks.remove(winner))
}
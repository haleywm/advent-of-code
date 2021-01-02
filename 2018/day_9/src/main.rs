use std::fs;
use regex::Regex;

fn main() {
    let in_parse = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();
    let input = in_parse.captures(&input).unwrap();
    let players: usize = input[1].parse().unwrap();
    // First marble that starts with 0 and not placed by a player not included
    let marbles: usize = input[2].parse().unwrap();
    // Using a vector as a linked list, where each value points to the item that comes after it
    let mut circle = Vec::with_capacity(marbles + 1);
    circle.push(0);
    // And a second vector for backwards direction
    let mut circle_rev = Vec::with_capacity(marbles + 1);
    circle_rev.push(0);
    let mut cur = 0;

    // Initializing this one
    let mut scores = vec![0; players];
    let mut cur_player = 0;

    // Running game
    for next in 1..=marbles {
        if next % 23 == 0 {
            // Special marble
            // Updating player
            cur_player = (cur_player + 23) % players;

            // Because of the single direction nature, going to have to loop through to find the item 7 before
            // Should be made up for by general speed gain at every other task
            let mut before = 0;
            let mut to_take = circle[cur];
            for i in 0..=8 {
                if i < 8 {
                    to_take = circle_rev[to_take];
                }
                else {
                    before = circle_rev[to_take];
                }
            }
            // Increasing score
            //println!("{} {}", next, to_take);
            scores[cur_player] += next + to_take;
            // Removing to_take from mapped values
            circle[before] = circle[to_take];
            circle_rev[circle[to_take]] = circle[before];
            cur = circle[to_take];
            // And inserting an empty value to maintain index
            circle.push(0);
            circle_rev.push(0);
        }
        else {
            // Inserting the next marble 2 pieces after the current one
            let before = circle[cur];
            let after = circle[before];

            circle[before] = next;
            // Pushing new value to position next
            circle.push(after);
            // Inserting value for reverse
            circle_rev[after] = next;
            circle_rev.push(before);
            // Shifting cur
            cur = next;
        }
    }
    // Printing high score
    let high = scores.into_iter().max().unwrap();
    println!("{}", high);
}

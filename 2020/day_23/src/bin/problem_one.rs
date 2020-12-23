use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cups = Vec::new();
    for label in input.chars() {
        cups.push(label.to_digit(10).unwrap() as i32);
    }
    const MOVES: usize = 100;

    assert!(cups.len() >= 5);

    let mut cur_pos = 0;
    for _ in 0..MOVES {
        let mut next_cups = Vec::with_capacity(3);
        for _ in 0..3 {
            let mut remove_pos = cur_pos + 1;
            if remove_pos >= cups.len() {
                remove_pos = 0;
                cur_pos -= 1;
            }
            next_cups.push(cups.remove(remove_pos));
        }
        let cur = &cups[cur_pos];
        //
        let min = cups.iter().enumerate().min_by_key(|x| {
            let score = (cur - 1) - x.1;
            if score >= 0 {
                score
            }
            else {
                // The number is too high don't want it
                100
            }
        }).unwrap();
        let dest = if *min.1 < *cur {
            // If the smallest value is less than the current label, return it's index
            min.0
        }
        else {
            // Otherwise, get the index of the largest value
            cups.iter().enumerate().max_by_key(|x| x.1).unwrap().0
        } + 1;

        

        for insert in next_cups.into_iter().rev() {
            cups.insert(dest, insert);
        }
        // Adjust the current position if inserted before the pos
        if dest <= cur_pos {
            cur_pos = (cur_pos + 3) % cups.len();
        }
        cur_pos = (cur_pos + 1) % cups.len();
    }
    // Lastly, printing the numbers as specified
    let mut idx = cups.iter().enumerate().find(|x| *x.1 == 1).unwrap().0 + 1;
    for _ in 1..cups.len() {
        print!("{}", cups[idx]);
        idx = (idx + 1) % cups.len();
    }
    println!();
}

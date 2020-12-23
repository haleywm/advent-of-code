use std::fs;
use std::collections::LinkedList;

fn main() {
    const CUPS: usize = 1000000;
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cups = LinkedList::new();
    for label in input.chars() {
        cups.push_back(label.to_digit(10).unwrap() as i32);
    }
    for i in (cups.len() + 1)..=CUPS {
        cups.push_back(i as i32);
    }
    println!("Finished placing cups");

    const MOVES: usize = 10000000;

    assert!(cups.len() >= 5);

    let max_val = CUPS as i32;
    
    let mut last_dest: Option<usize> = None;

    for go in 0..MOVES {
        if go % (MOVES / 100) == 0 {
            println!("{}%", go * 100 / MOVES);
        }

        let cur = cups.pop_front().unwrap();
        let mut next_cups = LinkedList::new();
        for _ in 0..3 {
            next_cups.push_back(cups.pop_front().unwrap());
        }
        
        let mut goal = 0;
        'goal_find: for i in 1..=3 {
            if cur - i < 1 {
                // Looking for something larger instead
                for j in 0..=3 {
                    if !next_cups.contains(&(max_val - j)) && cur != max_val - j {
                        // Found goal
                        goal = max_val - j;
                        break 'goal_find;
                    }
                }
            }

            if !next_cups.contains(&(cur - i)) {
                // Found goal
                goal = cur - i;
                break;
            }
        }
        let dest = if last_dest.unwrap_or(CUPS) > CUPS / 2 {
            // Last thing was on the larger end of the scale, so look in reverse order
            cups.iter().enumerate().rev().find(|x| *x.1 == goal).unwrap().0 + 1
        }
        else {
            cups.iter().enumerate().find(|x| *x.1 == goal).unwrap().0 + 1
        };
        last_dest = Some(dest);

        let mut list_back = cups.split_off(dest);
        cups.append(&mut next_cups);
        cups.append(&mut list_back);
        //println!("{}, {}", goal, dest);
        
        cups.push_back(cur);
    }
    //println!("{:?}", cups);
    // Lastly, printing the numbers as specified
    let mut scan = cups.into_iter().cycle();
    // Forwarding to where 1 is
    scan.find(|x| *x == 1);
    
    let val_one = scan.next().unwrap();
    let val_two = scan.next().unwrap();
    println!("{} * {} = {}", val_one, val_two, val_one * val_two);
}

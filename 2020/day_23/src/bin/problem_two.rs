use std::fs;

fn main() {
    const CUPS: usize = 1000000;
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cups = vec![0; CUPS + 1];
    {
        let mut prev = 0;
        // Index 0 points to the start of the list, every other index shows what comes after that number
        let mut total = 0;
        for label in input.chars() {
            let val = label.to_digit(10).unwrap() as usize;
            cups[prev] = val;
            prev = val;
            total += 1;
        }
        // If there's extra cups to insert, make the last number in the list point to the beginning of these numbers
        if total <= CUPS {
            cups[prev] = total + 1;
        }
        // Otherwise, make it point back to the beginning
        else {
            cups[prev] = cups[0];
        }
        for i in (total + 1)..CUPS {
            cups[i] = i + 1;
        }
        // Lastly, making the last number wrap around
        if total <= CUPS {
            cups[CUPS] = cups[0];
        }
    }

    const MOVES: usize = 10000000;

    // The current number we're going with
    let mut cur = cups[0];

    for _ in 0..MOVES {
        
        let val_one = cups[cur];
        let val_two = cups[val_one];
        let val_thr = cups[val_two];

        let mut goal = 0;
        for i in 1..=4 {
            if cur.saturating_sub(i) < 1 {
                // Looking for something larger instead
                for j in 0..=3 {
                    let comp = CUPS - j;
                    if val_one != comp && val_two != comp && val_thr != comp && cur != comp {
                        goal = comp;
                        break;
                    }
                }
                break;
            }

            let comp = cur - i;
            if val_one != comp && val_two != comp && val_thr != comp {
                // Found goal
                goal = comp;
                break;
            }
        }
        assert_ne!(goal, 0);

        
        // Found it
        cups[cur] = cups[val_thr];
        cups[val_thr] = cups[goal];
        cups[goal] = val_one;

        cur = cups[cur];
    }
    
    // Lastly, printing output
    let val_one = cups[1];
    let val_two = cups[val_one];

    let result = val_one as u64 * val_two as u64;
    println!("{} * {}: {}", val_one, val_two, result);
}

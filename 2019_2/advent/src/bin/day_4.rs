use std::fs;

fn main() {
    let input = fs::read_to_string("input/day_4.txt")
        .expect("Unable to read input");
    
    let input = input.trim();
    
    let pos = input.find('-').expect("Invalid input");
    let (start, end) = (&input[..pos], &input[pos + 1..]);
    let start: i32 = start.parse().expect("Invalid first number");
    let end: i32 = end.parse().expect("Invalid second number");

    // Now that input is parsed
    let valid = (start..=end)
        .filter(|&x| {
            let mut prev_dig = x % 10;
            let mut div = 10;
            let mut doubles = false;
            
            while div < x {
                let dig = (x / div) % 10;
                if dig == prev_dig {
                    // Adjacent doubles
                    doubles = true;
                }
                // Digits shouldn't decrease from left to right
                if dig > prev_dig {
                    return false;
                }

                prev_dig = dig;
                div *= 10;
            }

            // If it came this far then decrease rule met, only doubles
            doubles
        })
        .count();

    println!("Part 1: {}", valid);

    let valid = (start..=end)
        .filter(|&x| {
            let mut prev_dig = x % 10;
            let mut div = 10;
            let mut prev_double_count = 0;
            let mut doubles = false;
            
            while div < x * 10 {
                let dig = (x / div) % 10;
                if dig == prev_dig {
                    // Adjacent doubles
                    prev_double_count += 1;
                }
                else {
                    if prev_double_count == 1 {
                        doubles = true;
                    }
                    prev_double_count = 0;
                }
                // Digits shouldn't decrease from left to right
                if dig > prev_dig {
                    return false;
                }

                prev_dig = dig;
                div *= 10;
            }

            // If it came this far then decrease rule met, only doubles
            doubles
        })
        .count();

        println!("Part 2: {}", valid);
}
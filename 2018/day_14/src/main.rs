use std::fs;

fn main() {
    // The number of recipies to produce
    let input = fs::read_to_string("input.txt").unwrap().parse::<usize>().unwrap();
    let count = input + 10;
    let input_len = (input as f32).log10() as usize + 1;
    const REC_COUNT: usize = 2;

    let mut recipies: Vec<u8> = Vec::with_capacity(count);
    recipies.push(3);
    recipies.push(7);

    // Creating array to store the position of each recipe, inital val 0 and counting
    let mut positions: Vec<usize> = (0..REC_COUNT).collect();

    let mut answer_one: Option<usize> = None;
    let mut answer_two: Option<usize> = None;

    while answer_one.is_none() || answer_two.is_none() {
        // Summing each
        let mut total = 0;
        for pos in positions.iter() {
            total += recipies[*pos];
        }
        // Then appending as many as needed
        let to_add = (total as f32).log10() as u32;
        for factor in (0..=to_add).rev() {
            let next = (total / 10u8.pow(factor)) % 10;
            recipies.push(next);
        }
        // The finally, updating positions
        for pos in positions.iter_mut() {
            *pos = (*pos + recipies[*pos] as usize + 1) % recipies.len();
        }
        if answer_one.is_none() && recipies.len() >= count {
            for i in count-10..count {
                answer_one = Some(answer_one.unwrap_or(0) * 10 + recipies[i] as usize);
            }
        }
        if answer_two.is_none() && recipies.len() >= input_len {
            for offset in 0..=to_add {
                let offset = offset as usize;
                if recipies.len() >= input_len + offset {
                    let i = recipies.len() - offset - input_len;
                    let end = recipies.len() - offset;
                    let set = &recipies[i..end];
    
                    let mut total = 0;
                    for num in set.iter() {
                        total = total * 10 + *num as usize;
                    }
                    if total == input {
                        // Found result!
                        answer_two = Some(i);
                    }
                }
            }
        }
    }
    // Done! Printing the last 10 values
    println!("{}", answer_one.unwrap());
    println!("{}", answer_two.unwrap());

    // And then iterating through to find an occurance of the pattern

}

use std::env;

/*
 * Some notes:
 * So I have to find a number whose divisors plus the total sum to at least x/10.
 * Simplifying the given number a little I can divide it by 10 to work with just divisors,
 * and then half it to take the current house off, so I just have to find a number with
 * an aliquot sum greater than or equal to what I end up with.
*/
fn main() {
    let mut args = env::args();
    let target: usize = args.nth(1).and_then(|input| input.parse().ok()).unwrap_or(150);
    let limit = target / 10;
    let num = target / 10;

    // Making a rudimentary sieve:
    let mut sieve = Vec::with_capacity(limit);
    let mut lowest: Option<usize> = None;
    for _ in 0..limit {
        sieve.push(1);
    }

    for elf in 2..limit {
        for i in (elf..=limit).step_by(elf) {
            sieve[i - 1] += elf;
            if sieve[i - 1] >= num {
                if lowest.is_none() || lowest.unwrap() > i {
                    lowest = Some(i);
                }
            }
        }
    }

    match lowest {
        None => println!("Failed to find value"),
        Some(num) => println!("Lowest value: {}", num),
    }

    // Part 2
    let num = target / 11;
    for i in 0..limit {
        sieve[i] = 1;
    }
    lowest = None;
    for elf in 2..limit {
        for i in (elf..=limit).step_by(elf).take(50) {
            sieve[i - 1] += elf;
            if sieve[i - 1] >= num {
                if lowest.is_none() || lowest.unwrap() > i {
                    lowest = Some(i);
                }
            }
        }
    }
    match lowest {
        None => println!("Failed to find value"),
        Some(num) => println!("Lowest value: {}", num),
    }
}

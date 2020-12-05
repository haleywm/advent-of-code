struct PasswordGen {
    cur: u32,
    max: u32
}

impl PasswordGen {
    fn new(cur: u32, max: u32) -> PasswordGen {
        PasswordGen { cur, max }
    }

    fn next_valid_pass(mut num: u32) -> u32 {
        // Finds the next valid passcode that is equal to or greater to the given number
        let len: u32 = ((num as f32).log10() + 1.0) as u32;

        let mut max = 0;
        let mut prev = 0;
        let mut match_len = 1;
        let mut double = false;
        for i in (0..len).rev() {
            let mut cur = (num / 10u32.pow(i)) % 10;
            // Iterating through each digit in the number, from high to low
            if cur > max {
                max = cur;
            }
            else if cur < max {
                // Have to increase this digit
                num += (max - cur) * 10u32.pow(i);
                cur = max;
            }
            // Checking if a double naturally occurs while doing other operations
            if cur == prev {
                match_len += 1;
            }
            else {
                if match_len == 2 {
                    double = true;
                }
                match_len = 1;
            }
            // If at the last number, and no double, increase the number before to create double
            if !(double || match_len == 2) && i == 0 {
                if match_len == 1 {
                    // This number is by itself, so increasing the previous number to match is enough
                    num += (cur - prev) * 10;
                }
                else {
                    // This number is part of a larger chain, so have to increase the last and second last number to a higher number
                    if cur != 9 {
                        // Ez
                        num += 11;
                    }
                    else {
                        // Number ends in 999(or more), being lazy and using recursion
                        num = Self::next_valid_pass(num + 1);
                    }
                }
            }

            prev = cur;
        }

        num
    }
}

impl Iterator for PasswordGen {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur <= self.max {
            // Incrementing to next max val
            self.cur = Self::next_valid_pass(self.cur);
            if self.cur <= self.max {
                // Incrementing the current value so the next call increases it if needed
                self.cur += 1;
                Some(self.cur - 1)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}

fn validate_num(input: u32) -> bool {
    let mut double = false;
    let mut prev = 0;
    let mut max = 0;
    let mut match_chain = 1;
    let mut cur;
    for i in (0..6).rev() {
        cur = (input / 10u32.pow(i)) % 10;
        if cur < max {
            return false;
        }
        else {
            max = cur;
        }
        if cur == prev {
            match_chain += 1;
        }
        else {
            if match_chain == 2 {
                double = true;
            }
            match_chain = 1;
        }
        
        if i == 0 {
            // At end of chain
            if !(double || match_chain == 2) {
                return false;
            }
        }
        prev = cur;
    }
    
    true
}

fn main() {
    // let passwords = PasswordGen::new(382345, 843167);

    // let result = passwords.count();
    let result = (382345..843167).filter(|x| validate_num(*x)).count();
    for i in (0..6).rev() {
        println!("{}", i);
    }
    println!("{}", result);
}

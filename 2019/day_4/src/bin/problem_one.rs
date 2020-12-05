struct PasswordGen {
    cur: u32,
    max: u32
}

impl PasswordGen {
    fn new(cur: u32, max: u32) -> PasswordGen {
        // Subtracting 1 in case the starting value is also a valid number
        PasswordGen { cur, max }
    }

    fn next_valid_pass(mut num: u32) -> u32 {
        // Finds the next valid passcode that is equal to or greater to the given number
        let len: u32 = ((num as f32).log10() + 1.0) as u32;

        let mut max = 0;
        let mut prev = 0;
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
                double = true;
            }
            // If at the last number, and no double, increase the number before to create double
            if !double && i == 0 {
                num += (cur - prev) * 10;
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

fn main() {
    let passwords = PasswordGen::new(382345, 843167);

    let result = passwords.count();

    println!("{}", result);
}

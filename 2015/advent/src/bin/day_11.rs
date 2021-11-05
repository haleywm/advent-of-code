use std::env;

fn main() {
    let raw_pass = env::args().nth(1).unwrap_or(String::from("abcdefgh")).into_bytes();
    let pass = Password::new(raw_pass);
    let mut pass_gen = pass.filter(test_pass);
    let next_pass = pass_gen.next().expect("Unable to find next password");
    let next_pass = String::from_utf8_lossy(&next_pass);
    println!("{}", next_pass);
    let next_pass = pass_gen.next().expect("Unable to find next password");
    let next_pass = String::from_utf8_lossy(&next_pass);
    println!("{}", next_pass);
}

struct Password {
    pass: Vec<u8>,
}

impl Password {
    pub fn new(pass: Vec<u8>) -> Password {
        assert_eq!(pass.len(), 8);
        Password {pass}
    }
    
    fn increment_pass(&mut self) -> bool {
        let mut pos = self.pass.len() - 1;
        loop {
            let new_letter = Self::increment_byte(self.pass[pos]);
            self.pass[pos] = new_letter;
            // Checking if new_letter is 'a', and if we haven't reached the last letter
            if new_letter == b'a' && pos > 0 {
                pos -= 1;
            }
            else {
                // If we haven't, stop
                return pos > 0;
            }
        }
    }
    
    fn increment_byte(byte: u8) -> u8 {
        ((byte - 97 + 1) % 26) + 97
    }
}

impl Iterator for Password {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        Self::increment_pass(self).then(|| self.pass.clone())
    }
}

fn test_pass(pass: &Vec<u8>) -> bool {
    // Tests if a password meets the requirements, and returns true or false
    // Testing rule 1:
    let rule_one = pass
        .windows(3)
        .any(|win| win[0] + 1 == win[1] && win[1] + 1 == win[2]);
    let rule_two = !(pass.contains(&b'i') || pass.contains(&b'o') || pass.contains(&b'o'));
    let mut found_letter: Option<u8> = None;
    let mut rule_three = false;
    for win in pass.windows(2) {
        if win[0] == win[1] {
            // Match found
            match found_letter {
                None => found_letter = Some(win[0]),
                Some(cur) => {
                    if cur != win[0] {
                        rule_three = true;
                        break;
                    }
                }
            }
        }
    }

    rule_one && rule_two && rule_three
}

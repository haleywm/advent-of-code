use regex::RegexSet;

#[macro_use]
extern crate lazy_static;

static VALS: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid"
];
static REGEX_STRICT: &'static [&str] = &[
    r"\bbyr:(19[2-9][0-9]|200[0-2])\b",
    r"\biyr:20(1[0-9]|20)\b",
    r"\beyr:20(2[0-9]|30)\b",
    r"\bhgt:(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)\b",
    r"\bhcl:#[0-9a-f]{6}\b",
    r"\becl:(amb|blu|brn|gry|grn|hzl|oth)\b",
    r"\bpid:[0-9]{9}\b"
];

pub fn valid_passport(data: &str) -> bool {
    lazy_static! {
        static ref RE: RegexSet = RegexSet::new(
            VALS.iter()
            .map(|x| format!(r"\b{}:\S+", x))
            .collect::<Vec<String>>()
        ).unwrap();
    }

    RE.matches(data).iter().count() == VALS.len()
}

pub fn valid_passport_strict(data: &str) -> bool {
    lazy_static! {
        static ref RE: RegexSet = RegexSet::new(REGEX_STRICT).unwrap();
    }

    RE.matches(data).iter().count() == REGEX_STRICT.len()
}
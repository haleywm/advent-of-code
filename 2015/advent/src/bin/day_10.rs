use std::env;
use fancy_regex::Regex;
use lazy_static::lazy_static;


fn main() {
    let mut args = env::args();

    let mut number = args.nth(1).unwrap_or_else(|| String::from("1"));
    let iter_count: usize = args.next().and_then(|num| num.parse().ok()).unwrap_or(5);

    for _ in 0..iter_count {
        number = do_round(&number);
    }

    println!("Final Length: {}", number.len());
}

fn do_round(input: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d)\1*").unwrap();
    }

    let mut out = String::with_capacity(input.len() * 2);
    for cap in RE.captures_iter(input) {
        let cap = cap.unwrap();
        let num = cap.get(1).unwrap().as_str();
        let count = cap.get(0).unwrap().as_str().len().to_string();
        out.push_str(&count);
        out.push_str(num);
    }

    out
}

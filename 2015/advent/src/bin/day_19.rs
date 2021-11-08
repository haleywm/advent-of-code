use std::collections::HashSet;
use regex::Regex;

fn main() {
    let re = Regex::new(r"^(\w+) => (\w+)$").unwrap();

    let mut lines = advent::line_iter("input/day_19.txt")
        .expect("Unable to open input file")
        .map(|l| l.unwrap());
    
    // Take lines and parse them until an empty line is found
    let replacements: Vec<(String, String)> = lines
        .by_ref()
        .take_while(|line| line.len() > 0)
        .map(|line| {
            let cap = re.captures(&line).expect("Unable to parse line");
            let from = cap.get(1).unwrap().as_str().to_owned();
            let too = cap.get(2).unwrap().as_str().to_owned();
            (from, too)
        })
        .collect();
    
    // The next and last line in the iterator should be the medication name
    let med = lines.next().expect("Unable to read medication name");

    // Now to generate every possible variation
    let mut variations: HashSet<String> = HashSet::new();

    for (from, to) in replacements.iter() {
        // Rust doesn't have a built-in find iterator, but as the names can't have special regex chars I can just use regex
        let finder = Regex::new(&from).unwrap();
        for repr in finder.find_iter(&med) {
            let mut new = med.clone();
            new.replace_range(repr.range(), to);
            variations.insert(new);
        }
    }

    println!("Total possibilities: {}", variations.len());

    // Now to find a reverse
    // Cheated and looked the info up online, I should have studied the input a lot more :(
    let element_count = med.matches(|c: char| c.is_ascii_uppercase()).count();
    let rn_count = med.matches("Rn").count();
    let ar_count = med.matches("Ar").count();
    let y_count = med.matches("Y").count();
    let fin_tokens = replacements
        .iter()
        .filter_map(|(from, to)| {
            if from != "e" {
                None
            }
            else {
                Some(to.matches(|c: char| c.is_ascii_uppercase()).count())
            }
        })
        .min()
        .unwrap();
    let changes_needed = element_count - rn_count - ar_count - y_count * 2 - fin_tokens + 1;

    println!("Changes needed: {}", changes_needed);
}

use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
enum Activity {
    Wake,
    Sleep,
    Change(usize),
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Record {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    event: Activity,
}

fn main() {
    let parser = Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (falls asleep|wakes up|Guard #(\d+) begins shift)$").unwrap();
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    // First, I'll parse events so they can be sorted
    let mut events: Vec<Record> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let result = parser.captures(&line).unwrap();

        let year = result[1].parse().unwrap();
        let month = result[2].parse().unwrap();
        let day = result[3].parse().unwrap();
        let hour = result[4].parse().unwrap();
        let minute = result[5].parse().unwrap();

        let event = if result[6].starts_with("falls asleep") {
            Activity::Sleep
        }
        else if result[6].starts_with("wakes up") {
            Activity::Wake
        }
        else {
            // Must be a guard
            Activity::Change(result[7].parse().unwrap())
        };

        events.push(Record { year, month, day, hour, minute, event: event });
    }

    // Next, sorting
    events.sort_unstable();

    // Once it has been sorted, then getting the times people were asleep for
    // Hashmap maps guard ID's to sleep times
    let mut times: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    // Current guard
    let mut guard: Option<usize> = None;
    // When fell asleep
    let mut fell_asleep: Option<usize> = None;

    for item in events {
        match item.event {
            Activity::Change(id) => {
                guard = Some(id);
                fell_asleep = None;
            }
            Activity::Sleep => {
                fell_asleep = Some(item.minute);
            }
            Activity::Wake => {
                // Get prev values, panicing if they don't exist
                let sleep = fell_asleep.unwrap();
                let id = guard.unwrap();
                // Get the list for the guard, creating an empty one if needed, and inserting the sleep times
                let list = times.entry(id).or_insert_with(|| Vec::new());
                assert!(sleep < item.minute);
                list.push((sleep, item.minute));
            }
        }
    }

    // Lastly, for each guard, checking their most slept minute
    let (id, minute, _times) = times
        .iter()
        .map(|(&id, sleep_times)| {
            // Map to (id, sleepiest minute, times slept that minute)
            let (minute, time_slept) = (0..60)
                .map(|t| {
                    let mut total = 0;
                    for &(start, end) in sleep_times.iter() {
                        // Guards fall asleep at time start, and are awake at time end, so only the first can be equal
                        if start <= t && end > t {
                            total += 1
                        }
                    }
                    (t, total)
                })
                .max_by_key(|x| x.1 )
                .unwrap();
            (id, minute, time_slept)
        })
        .max_by_key(|x| x.2 )
        .unwrap();
    
    println!("{} * {} = {}", id, minute, id * minute);
}

use std::env;
use std::cmp;
use regex::Regex;

struct Reindeer {
    speed: i64,
    stamina: i64,
    rest: i64,
}

impl Reindeer {
    pub fn new(speed: i64, stamina: i64, rest: i64) -> Reindeer {
        Reindeer { speed, stamina, rest }
    }

    pub fn distance_travelled(&self, time: i64) -> i64 {
        // Calculates the distance that would be travelled by this raindeer in time
        let cycle = self.stamina + self.rest;
        let cycles = time / cycle;
        // Get the remaining run time, either the remaining run time, or the full stamina time
        let rem = cmp::min(time % cycle, self.stamina);

        (cycles * self.stamina + rem) * self.speed
    }
}

fn main() {
    let re = Regex::new(r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$").unwrap();
    let deer: Vec<Reindeer> = advent::line_iter("input/day_14.txt")
        .expect("Unable to open input file")
        .map(|line| {
            let line = line.unwrap();
            let cap = re.captures(&line).expect("Unable to parse line");
            let speed = cap.get(2).unwrap().as_str().parse().unwrap();
            let stamina = cap.get(3).unwrap().as_str().parse().unwrap();
            let rest = cap.get(4).unwrap().as_str().parse().unwrap();

            Reindeer::new(speed, stamina, rest)
        })
        .collect();
    
    let time = env::args().nth(1).and_then(|x| x.parse().ok()).unwrap_or(1000);
    
    let fastest = deer
        .iter()
        .map(|r| r.distance_travelled(time))
        .max()
        .unwrap();

    println!("{}", fastest);

    // Then the new scoring system
    let mut score: Vec<usize> = vec![0; deer.len()];
    for cur in 1..=time {
        let mut winner: Vec<(usize, i64)> = deer
            .iter()
            .map(|r| r.distance_travelled(cur))
            .enumerate()
            .collect();
        winner.sort_unstable_by_key(|x| -x.1);
        score[winner[0].0] += 1;
        let mut i = 1;
        while i < winner.len() && winner[i].1 == winner[0].1 {
            score[winner[i].0] += 1;
            i += 1;
        }
    }

    println!("{}", score.into_iter().max().unwrap());
}

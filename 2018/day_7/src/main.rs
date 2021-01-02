use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::min;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    // I will do this by creating a hashmap that maps chars to a vector of chars
    // Each char represents the step, and the vector represents uncompleted prerequisites
    // After reading input, I'll then move on to execution
    // I will perform x passes, where each pass will remove the completed steps from the vec list, and note the items with no further reqs
    // The first alphabetical free item in the list will then be used
    let mut stages: HashMap<u8, Vec<u8>> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        // Since lines have fixed sizes just read chars at certain points
        let req = line[5..6].as_bytes()[0];
        let step = line[36..37].as_bytes()[0];

        // Creating entry for rec to ensure that even steps with no prereqs are properly in the set
        stages.entry(req).or_insert_with(|| Vec::new());
        // Then creating an empty entry for step if needed, or getting the existing one if previously given in set
        let list = stages.entry(step).or_insert_with(|| Vec::new());
        list.push(req);
    }

    // Cloning stages for later use
    let mut stages_timed = stages.clone();
    
    // Task 1:
    {
        let mut order = String::with_capacity(stages.len());
        let mut last_done: Option<u8> = None;

        for _ in 0..stages.len() {
            // Removing the last done step and collecting everything with 0 remaining steps
            let mut can_do = Vec::new();
            for (step, reqs) in stages.iter_mut() {
                // Removing if it's in the array
                if let Some(rem) = last_done {
                    if let Some((index, _)) = reqs.iter().enumerate().find(|x| *x.1 == rem) {
                        reqs.swap_remove(index);
                    }
                }
                if reqs.len() == 0 {
                    can_do.push(*step);
                }
            }

            // Getting the alphabetically first thing in the list
            let to_do = can_do.into_iter().min().expect("Couldn't find any free steps");
            stages.remove(&to_do);
            order.push(to_do as char);
            last_done = Some(to_do);
        }

        println!("{}", order);
    }


    // Task 2:
    {
        const WORKER_COUNT: usize = 5;
        const TIME_ADD: usize = 60;

        
        let mut cur_workers = WORKER_COUNT;
        let mut cur_time = 0;
        // Contains currently in process tasks
        let mut tasks: Vec<(usize, u8)> = Vec::new();
        // Stores the completed tasks
        let mut done: Vec<u8> = Vec::new();

        let result = loop {
            // Removing the last done step and collecting everything with 0 remaining steps
            let mut can_do = Vec::new();
            for (step, reqs) in stages_timed.iter_mut() {
                // Removing if it's in the array
                if let Some((index, _)) = reqs.iter().enumerate().find(|x| done.contains(x.1)) {
                    reqs.swap_remove(index);
                }
                if reqs.len() == 0 {
                    can_do.push(*step);
                }
            }

            // Sorting the list alphabetically (in reverse so I can pop the first elements off) and starting as many tasks as possible
            can_do.sort_unstable_by(|a, b| b.cmp(a));
            let will_do = min(can_do.len(), cur_workers);
            for _ in 0..will_do {
                let to_do = can_do.pop().unwrap();
                stages_timed.remove(&to_do);
                assert!(to_do.is_ascii_uppercase());
                let time = cur_time + (to_do - 64) as usize + TIME_ADD;
                tasks.push((time, to_do));
                cur_workers -= 1;
            }

            assert_eq!(tasks.len(), WORKER_COUNT - cur_workers);

            // Then lastly advancing time to the next closest task
            // Putting the closest items at the end
            tasks.sort_by(|a, b| b.0.cmp(&a.0));
            // If the stages list is empty, I can just take the longest to do task and return that and call it a day
            if stages_timed.is_empty() {
                // (If the list is empty return current time instead)
                break tasks.into_iter().map(|x| x.0).min().unwrap_or(cur_time);
            }
            cur_time = tasks.last().unwrap().0;
            while !tasks.is_empty() && tasks.last().unwrap().0 == cur_time {
                let completed = tasks.pop().unwrap();
                done.push(completed.1);
                cur_workers += 1;
            }
        };
    println!("{}", result);
    }
}

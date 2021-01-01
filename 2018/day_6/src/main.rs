use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let points: Vec<(i32, i32)> = io::BufReader::new(file)
        .lines()
        .map(|x| {
            // Expecting two numbers separated by ", "
            let line = x.unwrap();
            let mut nums = line.split(", ");
            (nums.next().unwrap().parse().unwrap(), nums.next().unwrap().parse().unwrap())
        })
        .collect();
    
    // Find the 'max bounding box', the box where the min and max x and y values fall upon.
    let min_x = points.iter().map(|x| x.0).min().unwrap();
    let min_y = points.iter().map(|x| x.1).min().unwrap();
    let max_x = points.iter().map(|x| x.0).max().unwrap();
    let max_y = points.iter().map(|x| x.1).max().unwrap();
    
    let result = points.iter().map(|&cur| {
        // Mapping out points until all are found
        let mut total: usize = 0;
        let mut to_check = vec![cur];
        let mut checked = HashSet::new();
        checked.insert(cur);
        while let Some(next) = to_check.pop() {
            // If this point is outside the bounding box, it's infinite, move on
            if next.0 < min_x || next.0 > max_x || next.1 < min_y || next.1 > max_y {
                total = 0;
                break;
            }
            // Finding nearest neighbour
            let mut closest: Option<(i32, (i32, i32))> = None;
            let mut need_lower = false;
            for pos in points.iter() {
                let dist = (next.0 - pos.0).abs() + (next.1 - pos.1).abs();
                if closest.is_some() && closest.unwrap().0 == dist {
                    // Equal dist uh oh
                    need_lower = true;
                }
                else if closest.is_none() || closest.unwrap().0 > dist {
                    closest = Some((dist, *pos));
                    need_lower = false;
                }
            }
            if closest.unwrap().1 == cur && !need_lower {
                // Found match! Incrementing counter and checking neighbours if they aren't already being checked
                total += 1;
                for &pos in &[(next.0 - 1, next.1), (next.0 + 1, next.1), (next.0, next.1 - 1), (next.0, next.1 + 1)] {
                    if !checked.contains(&pos) {
                        checked.insert(pos);
                        to_check.push(pos);
                    }
                }
            }
        }
        total
    })
    .max()
    .unwrap();

    println!("{}", result);

    const MAX_DIST: i32 = 10000;

    // Just iterate over every possible value I don't care
    let result = (min_x..=max_x)
        .flat_map(|x| {
            (min_y..=max_y).map(move |y| (x, y))
        })
        .filter(|&(x, y)| {
            let mut total = 0;
            for point in points.iter() {
                total += (point.0 - x).abs() + (point.1 - y).abs();
            }
            total < MAX_DIST
        })
        .count();

    println!("{}", result);
}

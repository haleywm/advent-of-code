use std::collections::{HashMap, HashSet};

fn main() {
    let wires: Vec<(HashSet<(i32, i32)>, HashMap<(i32, i32), i32>)> = advent::line_iter("input/day_3.txt")
        .expect("Unable to open file")
        .map(|line| line.unwrap())
        .map(|line| {
            // Parse string into set of points
            let mut x = 0;
            let mut y = 0;
            let mut steps = 0;
            let mut point_set = HashSet::new();
            let mut point_map = HashMap::new();
            for step in line.split(',') {
                let (dir, num) = step.split_at(1);
                let dir = dir.chars().next().unwrap();
                let num: i32 = num.parse().unwrap();
                assert!(num > 0);

                for _ in 0..num {
                    steps += 1;
                    match dir {
                        'R' => x += 1,
                        'D' => y -= 1,
                        'L' => x -= 1,
                        'U' => y += 1,
                        unknown => panic!("Unrecognized direction: {}", unknown)
                    }
                    point_set.insert((x, y));
                    point_map.insert((x, y), steps);
                }
            }
            
            return (point_set, point_map);
        })
        .collect();

    assert_eq!(wires.len(), 2);

    let inter = wires[0].0.intersection(&wires[1].0);

    let min = inter
        .map(|x| x.0.abs() + x.1.abs())
        .min();

    match min {
        Some(dist) => println!("Part 1: {}", dist),
        None => println!("Failed to solve Part 1")
    }

    let inter = wires[0].0.intersection(&wires[1].0);

    let min = inter
        .map(|x| {
            let point_a = *wires[0].1.get(x).unwrap();
            let point_b = *wires[1].1.get(x).unwrap();

            point_a + point_b
        })
        .min();
    
    match min {
        Some(dist) => println!("Part 2: {}", dist),
        None => println!("Failed to solve Part 2")
    }
}
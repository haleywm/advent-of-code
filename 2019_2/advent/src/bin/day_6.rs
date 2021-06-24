use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, (Option<String>, Vec<String>)> = HashMap::new();

    map.insert(String::from("COM"), (None, Vec::new()));

    advent::line_iter("input/day_6.txt")
        .expect("Unable to read file")
        .for_each(|line| {
            let line = line.unwrap();
            let pos = line.find(')').expect("Invalid line");
            let parent = &line[..pos];
            let child = &line[pos + 1..];
            map.entry(parent.to_owned())
                .or_insert_with(|| (None, Vec::new()))
                .1
                .push(child.to_owned());
            
            map.entry(child.to_owned())
                .or_insert_with(|| (None, Vec::new()))
                .0 = Some(parent.to_owned());
        });
    
    // Now that I've built up the worlds worst graph, count orbits
    let mut to_check = Vec::new();
    to_check.push("COM");
    let mut total = 0;
    let mut depth = 0;
    while !to_check.is_empty() {
        let mut next_check: Vec<&str> = Vec::new();
        for cur in to_check.into_iter() {
            total += depth;
            let cur = map.get(cur).unwrap();
            for child in cur.1.iter() {
                next_check.push(child);
            }
        }
        depth += 1;
        to_check = next_check;
    }

    println!("Task 1: {}", total);

    let mut parents: Vec<&str> = Vec::new();
    let mut cur = map.get("YOU").expect("Expected 'YOU'");
    
    while let (Some(parent), _) = cur {
        parents.push(parent);
        cur = map.get(parent).unwrap();
    }
    // After getting all parents, find where SAN first shares a parent.
    cur = map.get("SAN").expect("Expected 'SAN'");
    let mut climb = 0;
    let mut descend: Option<usize> = None;
    while let (Some(parent), _) = cur {
        let res = parents
            .iter()
            .enumerate()
            .find(|(_, &x)| x.eq(parent));
        if let Some((pos, _)) = res {
            descend = Some(pos);
        }
        if parents.contains(&parent.as_str()) {
            break;
        }
        climb += 1;
        cur = map.get(parent).unwrap();
    }
    
    match descend {
        Some(descend) => println!("Task 2: {}", climb + descend),
        None => println!("Failed to solve task 2")
    }
}

use itertools::Itertools;

fn main() {
    let packages: Vec<i64> = advent::line_iter("input/day_24.txt")
        .expect("Error opening input")
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    
    let part_one = find_best_entanglement(packages.clone(), 3).unwrap();
    println!("{}", part_one);
    let part_two = find_best_entanglement(packages, 4).unwrap();
    println!("{}", part_two);
}

fn find_best_entanglement(packages: Vec<i64>, target: i64) -> Option<i64> {
    assert!(target >= 2);
    let total: i64 = packages.iter().sum();
    assert!(total % target == 0, "Total package weight {} not divisible by {}", total, target);
    let target_weight = total / target;

    // Finding a valid combination with the lowest number of items in one group
    // If multiple valid combinations within that group select the one with the lowest product
    for group_size in 1..=(packages.len() - target as usize + 1) {
        let mut candidates: Vec<Vec<i64>> = Vec::new();
        // Start off seeing if a 1 size group is possible, then 2, etc
        for group in packages.iter().combinations(group_size) {
            let group_total: i64 = group.iter().cloned().sum();
            if group_total == target_weight {
                // Possible win, checking if total can be made out of other numbers
                // This assumes there's no double numbers
                let other_total: i64 = packages
                    .iter()
                    .cloned()
                    .filter(|x| !group.contains(&x))
                    .sum();
                if other_total == (target - 1) * target_weight {
                    // Valid candidate!
                    candidates.push(group.into_iter().copied().collect());
                }
            }
        }
        // Seeing if any valid candidates found
        if !candidates.is_empty() {
            return candidates
                .into_iter()
                .map(|x| x.into_iter().product::<i64>())
                .min();
        }
    }
    None
}

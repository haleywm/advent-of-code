use std::cmp;
use regex::Regex;


#[allow(clippy::many_single_char_names)]
fn main() {
    let re = Regex::new(r"^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$").unwrap();
    let cookies: Vec<(i64, i64, i64, i64, i64)> = advent::line_iter("input/day_15.txt")
        .expect("Unable to open input file")
        .map(|line| {
            let line = line.unwrap();
            let cap = re.captures(&line).expect("Unable to parse line");
            let a = cap.get(2).unwrap().as_str().parse().unwrap();
            let b = cap.get(3).unwrap().as_str().parse().unwrap();
            let c = cap.get(4).unwrap().as_str().parse().unwrap();
            let d = cap.get(5).unwrap().as_str().parse().unwrap();
            let e = cap.get(6).unwrap().as_str().parse().unwrap();

            (a, b, c, d, e)
        })
        .collect();
    
    // Then to brute force the possibilities:
    let best = arrays_summing_to(100, cookies.len() as i64)
        .into_iter()
        .map(|combination| {
            let scores = cookies
                .iter()
                .zip(combination.into_iter())
                .map(|((a, b, c, d, _), count)| {
                    (a * count, b * count, c * count, d * count)
                })
                .fold((0, 0, 0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2, acc.3 + x.3));
            cmp::max(0, scores.0) * cmp::max(0, scores.1) * cmp::max(0, scores.2) * cmp::max(0, scores.3)
        })
        .max()
        .unwrap();
    
    println!("{}", best);

    // Then to brute force the possibilities:
    let best_calories = arrays_summing_to(100, cookies.len() as i64)
        .into_iter()
        .filter_map(|combination| {
            let scores = cookies
                .iter()
                .zip(combination.into_iter())
                .map(|((a, b, c, d, e), count)| {
                    (a * count, b * count, c * count, d * count, e * count)
                })
                .fold((0, 0, 0, 0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2, acc.3 + x.3, acc.4 + x.4));
            if scores.4 == 500 {
                Some(cmp::max(0, scores.0) * cmp::max(0, scores.1) * cmp::max(0, scores.2) * cmp::max(0, scores.3))
            }
            else {
                None
            }
        })
        .max()
        .unwrap();
    
    println!("{}", best_calories);
}

fn arrays_summing_to(total: i64, num_values: i64) -> Vec<Vec<i64>> {
    if num_values <= 1 {
        vec![vec![total]]
    }
    else {
        let mut full = Vec::new();
        for i in 0..=total {
            let mut sub = arrays_summing_to(total - i, num_values - 1);
            for set in sub.iter_mut() {
                set.push(i);
            }
            full.append(&mut sub);
        }
        full
    }
}

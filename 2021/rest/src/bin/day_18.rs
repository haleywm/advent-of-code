use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Snailfish {
    Num(i64),
    OpenBracket,
    CloseBracket,
}

fn main() {
    let file = fs::File::open("input/day_18.txt").expect("Invalid Filename");

    let items: Vec<VecDeque<Snailfish>> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            parse_snailfish(&line)
        })
        .collect();

    // Part 1
    let sum = items
        .iter()
        .cloned()
        .reduce(|acc, cur| add_snailfish(acc, cur))
        .unwrap();

    println!("{}", get_magnitude(&sum));

    // Part 2
    let max = items
        .iter()
        .permutations(2)
        .map(|x| get_magnitude(&add_snailfish(x[0].clone(), x[1].clone())))
        .max()
        .unwrap();

    println!("{}", max);
}

fn parse_snailfish(line: &str) -> VecDeque<Snailfish> {
    line.chars()
        .filter_map(|x| match x {
            '[' => Some(Snailfish::OpenBracket),
            ']' => Some(Snailfish::CloseBracket),
            ',' => None,
            x => Some(Snailfish::Num(x.to_digit(10).unwrap() as i64)),
        })
        .collect()
}

fn add_snailfish(mut a: VecDeque<Snailfish>, mut b: VecDeque<Snailfish>) -> VecDeque<Snailfish> {
    a.append(&mut b);
    a.push_front(Snailfish::OpenBracket);
    a.push_back(Snailfish::CloseBracket);

    reduce_snailfish(&mut a);

    a
}

fn reduce_snailfish(list: &mut VecDeque<Snailfish>) {
    while explode_snailfish(list) || split_snailfish(list) {}
}

fn explode_snailfish(list: &mut VecDeque<Snailfish>) -> bool {
    let mut depth = 0;

    for (pos, cur) in list.iter().enumerate() {
        match cur {
            Snailfish::OpenBracket => depth += 1,
            Snailfish::CloseBracket => depth -= 1,
            Snailfish::Num(_) => {
                if depth > 4 && matches!(list[pos + 1], Snailfish::Num(_)) {
                    let left = match list[pos] {
                        Snailfish::Num(x) => x,
                        _ => panic!("Uh oh"),
                    };
                    let right = match list[pos + 1] {
                        Snailfish::Num(x) => x,
                        _ => panic!("Uh oh"),
                    };
                    assert_eq!(list.remove(pos - 1), Some(Snailfish::OpenBracket));
                    assert_eq!(list.remove(pos + 1), Some(Snailfish::CloseBracket));
                    list.remove(pos);
                    list[pos - 1] = Snailfish::Num(0);
                    let to_skip = list.len() - pos + 1;
                    for item in list.iter_mut().rev().skip(to_skip) {
                        if let Snailfish::Num(val) = item {
                            *item = Snailfish::Num(*val + left);
                            break;
                        }
                    }
                    for item in list.iter_mut().skip(pos) {
                        if let Snailfish::Num(val) = item {
                            *item = Snailfish::Num(*val + right);
                            break;
                        }
                    }
                    return true;
                }
            }
        }
    }

    false
}

fn split_snailfish(list: &mut VecDeque<Snailfish>) -> bool {
    for (i, cur) in list.iter().enumerate() {
        if let Snailfish::Num(x) = *cur {
            if x > 9 {
                let left = x / 2;
                let right = x - left;

                list[i] = Snailfish::Num(left);
                list.insert(i + 1, Snailfish::Num(right));
                list.insert(i, Snailfish::OpenBracket);
                list.insert(i + 3, Snailfish::CloseBracket);
                return true;
            }
        }
    }

    false
}

fn get_magnitude(list: &VecDeque<Snailfish>) -> i64 {
    let mut stack = Vec::new();
    for item in list {
        match *item {
            Snailfish::Num(x) => stack.push(x),
            Snailfish::CloseBracket => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(2 * a + 3 * b);
            }
            _ => {}
        }
    }

    assert_eq!(stack.len(), 1);
    stack.pop().unwrap()
}

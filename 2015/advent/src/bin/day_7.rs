use std::collections::hash_map::HashMap;
use std::env;
use regex::Regex;

#[derive(Debug)]
enum Value {
    Literal(u16),
    Reference(String),
}

#[derive(Debug)]
enum Gate {
    Direct(Value),
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, u16),
    RShift(Value, u16),
    Not(Value),
}

fn main() {
    let re = Regex::new(r"^([0-9a-z]*) ?(AND|OR|LSHIFT|RSHIFT|NOT)? ?([0-9a-z]*) -> ([a-z]+)$").unwrap();
    let mut values: HashMap<String, Gate> = advent::line_iter("input/day_7.txt")
        .expect("Unable to open file!")
        .map(|line| {
            let line = line.unwrap();
            let cap = re.captures(&line).unwrap();

            let gate = match cap.get(2) {
                None => {
                    // Just a direct definition
                    Gate::Direct(parse_ref(cap.get(1).unwrap().as_str()))
                },
                Some(instruction) => {
                    let instruction = instruction.as_str();
                    match instruction {
                        "AND" => {
                            Gate::And(
                                parse_ref(cap.get(1).unwrap().as_str()),
                                parse_ref(cap.get(3).unwrap().as_str()),
                            )
                        },
                        "OR" => {
                            Gate::Or(
                                parse_ref(cap.get(1).unwrap().as_str()),
                                parse_ref(cap.get(3).unwrap().as_str()),
                            )
                        },
                        "LSHIFT" => {
                            Gate::LShift(
                                parse_ref(cap.get(1).unwrap().as_str()),
                                cap.get(3).unwrap().as_str().parse::<u16>().unwrap(),
                            )
                        },
                        "RSHIFT" => {
                            Gate::RShift(
                                parse_ref(cap.get(1).unwrap().as_str()),
                                cap.get(3).unwrap().as_str().parse::<u16>().unwrap(),
                            )
                        },
                        "NOT" => {
                            Gate::Not(
                                parse_ref(cap.get(3).unwrap().as_str())
                            )
                        },
                        x => panic!("Unrecognized instruction: {}", x),
                    }
                },
            };
            (cap.get(4).unwrap().as_str().to_owned(), gate)
        })
        .collect();
    
    let mut to_resolve: Vec<String> = values
        .keys()
        .cloned()
        .collect();
    
    while !to_resolve.is_empty() {
        let mut i = 0;
        while i < to_resolve.len() {
            let key = &to_resolve[i];
            if let Some(num) = eval_key(&values, key) {
                *values.get_mut(key).unwrap() = Gate::Direct(Value::Literal(num));
                to_resolve.remove(i);
            }
            else {
                i += 1;
            }
        }
    }

    let target = env::args().nth(1);
    
    match target {
        Some(target) => println!("{}", eval_key(&values, &target).unwrap()),
        None => for key in values.keys() {
            println!("{}: {}", key, eval_key(&values, key).unwrap());
        }
    };
}

fn parse_ref(raw: &str) -> Value {
    raw.parse::<u16>().map_or_else(|_| Value::Reference(raw.to_owned()), |num| Value::Literal(num))
}

fn eval_key(map: &HashMap<String, Gate>, key: &String) -> Option<u16> {
    let gate = map.get(key).unwrap();

    match gate {
        Gate::Direct(val) => eval_value(map, val),
        Gate::And(val_a, val_b) => {
            let val_a = eval_value(map, val_a)?;
            let val_b = eval_value(map, val_b)?;
            Some(val_a & val_b)
        },
        Gate::Or(val_a, val_b) => {
            let val_a = eval_value(map, val_a)?;
            let val_b = eval_value(map, val_b)?;
            Some(val_a | val_b)
        },
        Gate::LShift(val, shift) => {
            let val = eval_value(map, val)?;
            Some(val << *shift)
        },
        Gate::RShift(val, shift) => {
            let val = eval_value(map, val)?;
            Some(val >> *shift)
        },
        Gate::Not(val) => Some(!(eval_value(map, val)?))
    }
}

fn eval_value(map: &HashMap<String, Gate>, val: &Value) -> Option<u16> {
    match val {
        Value::Literal(num) => Some(*num),
        Value::Reference(key) => {
            if let Some(Gate::Direct(Value::Literal(num))) = map.get(key) {
                Some(*num)
            }
            else {
                None
            }
        }
    }
}

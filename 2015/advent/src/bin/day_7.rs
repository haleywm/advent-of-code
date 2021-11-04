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
    let values: HashMap<String, Gate> = advent::line_iter("input/day_7.txt")
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
    
    let target = env::args().nth(1);
    
    match target {
        Some(target) => println!("{}", eval_key(&values, &target)),
        None => for key in values.keys() {
            println!("{}: {}", key, eval_key(&values, key));
        }
    };
}

fn parse_ref(raw: &str) -> Value {
    raw.parse::<u16>().map_or_else(|_| Value::Reference(raw.to_owned()), |num| Value::Literal(num))
}

fn eval_key(map: &HashMap<String, Gate>, key: &String) -> u16 {
    let gate = map.get(key).unwrap();

    match gate {
        Gate::Direct(val) => eval_value(map, val),
        Gate::And(val_a, val_b) => {
            let val_a = eval_value(map, val_a);
            let val_b = eval_value(map, val_b);
            val_a & val_b
        },
        Gate::Or(val_a, val_b) => {
            let val_a = eval_value(map, val_a);
            let val_b = eval_value(map, val_b);
            val_a | val_b
        },
        Gate::LShift(val, shift) => {
            let val = eval_value(map, val);
            val << *shift
        },
        Gate::RShift(val, shift) => {
            let val = eval_value(map, val);
            val >> *shift
        },
        Gate::Not(val) => !eval_value(map, val)
    }
}

fn eval_value(map: &HashMap<String, Gate>, val: &Value) -> u16 {
    match val {
        Value::Literal(num) => *num,
        Value::Reference(key) => eval_key(map, key)
    }
}

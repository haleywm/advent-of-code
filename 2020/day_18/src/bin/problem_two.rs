use std::fs;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
enum Symbol {
    Num(i64),
    Add,
    Mult,
    BrackOpen,
    BrackClose,
}

use Symbol::*;

fn main() {
    let re = Regex::new(r"\d+|\S").unwrap();
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    // Each line should be a vector of characters
    let lines = io::BufReader::new(file)
        .lines()
        .map(|x| {
            let mut val: Vec<Symbol> = Vec::new();
            for item in re.captures_iter(&x.unwrap()) {
                val.push(
                    match &item[0] {
                        "+" => Add,
                        "*" => Mult,
                        "(" => BrackOpen,
                        ")" => BrackClose,
                        num => match num.parse::<i64>() {
                            Ok(val) => Num(val),
                            Err(_) => panic!("Unrecognized input {}", num),
                        }
                    }
                );
            }
            val
        });
    
    // Now that line has been parsed, performing operations on each one
    let mut result = 0;
    for equation in lines {
        let mut num_stack: Vec<i64> = Vec::new();
        let mut op_stack: Vec<Symbol> = Vec::new();
        for item in equation {
            match item {
                Num(val) => {
                    // Found a number
                    // Seeing if any math can be done to it
                    // If the stack has anything on it that isn't an open bracket
                    if let Some(Add) = op_stack.last() {
                        op_stack.pop();
                        let num = num_stack.pop().unwrap() + val;
                        num_stack.push(num);
                    }
                    else {
                        // Put the number on the stack for later
                        num_stack.push(val);
                    }
                }
                operation @ Add | operation @ Mult | operation @ BrackOpen => {
                    op_stack.push(operation);
                }
                BrackClose => {
                    // There may be multiple mutliplications on the stack doing all of them
                    while let Some(Mult) = op_stack.last() {
                        op_stack.pop();
                        let num = num_stack.pop().unwrap() * num_stack.pop().unwrap();
                        num_stack.push(num);
                    }
                    // The last thing on the stack at this point should be an open bracket, remove that, and then if the next thing is an addition do that
                    assert_eq!(Some(BrackOpen), op_stack.pop());
                    if let Some(Add) = op_stack.last() {
                        op_stack.pop();
                        let num = num_stack.pop().unwrap() + num_stack.pop().unwrap();
                        num_stack.push(num);
                    }
                }
            }
        }
        // Lastly, closing any remaining mults should they exist
        while let Some(Mult) = op_stack.last() {
            op_stack.pop();
            let num = num_stack.pop().unwrap() * num_stack.pop().unwrap();
            num_stack.push(num);
        }
        // Finally done, there should be nothing in the op stack, and 1 number in the num stack
        assert_eq!(0, op_stack.len());
        assert_eq!(1, num_stack.len());
        result += num_stack.pop().unwrap();
    }
    println!("{}", result);
}

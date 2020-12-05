use day_7::{run_intcode, string_to_intcode, OwnOrRef::*};
use itertools::Itertools;
use std::fs;
use std::io::{self, BufRead};
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut line = String::new();
    io::BufReader::new(file)
        .read_line(&mut line)
        .expect("Couldn't read file");
    let intcode = string_to_intcode(line.trim()).unwrap();

    let result = (5..10)
        .permutations(5)
        .map(|x| {
            // Running 5 asynchronous interpreters and running input between them
            // Final answer is the input to the first modulator, and will be mutex'd so that I can get it back afterwards.
            // After execution there should be 1 number remaining, the final answer.
            let (first_in, final_answer) = mpsc::channel::<i32>();
            first_in.send(x[0]).unwrap();
            first_in.send(0).unwrap();

            let final_answer = Arc::new(Mutex::new(final_answer));

            let mut prev_out = Ref(Arc::clone(&final_answer));

            for phase in x.iter().skip(1) {
                // Use prev_out, the previous nodes output,
                // and phase, which indicates the phase on the next one in line,
                // to create a new reciever pair and processor
                let (next_in, cur_out) = mpsc::channel::<i32>();
                next_in.send(*phase).unwrap();
                run_intcode(intcode.clone(), prev_out, Own(next_in));
                prev_out = Own(cur_out);
            }
            // Lastly, creating Amp E and attaching it's output to first_in
            let last_amp = run_intcode(intcode.clone(), prev_out, Own(first_in));
            // Waiting for it to finish
            last_amp.join().unwrap();
            let final_answer = final_answer.lock().unwrap();
            final_answer.recv().unwrap()
        })
        .max()
        .unwrap();
    println!("{}", result);
    //run_intcode(line.trim()).join().unwrap();
}

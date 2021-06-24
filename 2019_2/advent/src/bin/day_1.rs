use advent;

fn main() {
    let mut sum_one: u32 = 0;
    let mut sum_two: u32 = 0;
    advent::line_iter("input/day_1.txt")
        .expect("Unable to open file")
        .map(|x| x.unwrap().parse::<u32>().expect("Invalid character"))
        .for_each(|x| {
            let mut fuel = (x / 3).saturating_sub(2);
            sum_one += fuel;
            while fuel != 0 {
                sum_two += fuel;
                fuel = (fuel / 3).saturating_sub(2);
            }
        });

    println!("Task 1: {}", sum_one);
    println!("Task 2: {}", sum_two);
}
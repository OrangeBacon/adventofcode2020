use std::fs;
use std::env;
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;

type Solution = fn(String) -> ();

fn main() {
    let solutions: Vec<Solution> = vec![day01::day01, day02::day02, day03::day03,
        day04::day04];

    let args: Vec<String> = env::args().collect();
    let mut args_used = 1;

    let day_number = match args.get(args_used) {
        Some(arg1) => {
            if let Ok(num) = arg1.parse::<usize>() {
                args_used += 1;
                if num <= solutions.len() {
                    num
                } else {
                    panic!("Invalid day number");
                }
            } else {
                solutions.len()
            }
        }
        None => solutions.len(),
    };

    let path_str = match args.get(args_used) {
        Some(arg2) => arg2.clone(),
        None => format!("data/day{}.txt", day_number),
    };

    let contents = match fs::read_to_string(&path_str) {
        Err(why) => panic!("Couldn't read file {}: {}", path_str, why),
        Ok(f) => f,
    };

    println!("Running day {} on {}", day_number, path_str);
    let now = Instant::now();
    solutions[day_number - 1](contents);
    println!("Run in {}ms", now.elapsed().as_millis());
}

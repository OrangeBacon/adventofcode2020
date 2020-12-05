use std::env;
use std::fs;
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

type Solution = fn(String) -> ();

fn print_time(mut num: f64) {
    let mut order = 0;
    while num < 1.0 {
        num *= 1000.0;
        order += 1;
    }

    println!(
        "{:.3}{}",
        num,
        match order {
            0 => "s",
            1 => "ms",
            2 => "μs",
            3 => "ns",
            _ => "?",
        }
    );
}

fn run_solution(solution: Solution, path: &str) {
    let contents = match fs::read_to_string(&path) {
        Err(why) => panic!("Couldn't read file {}: {}", path, why),
        Ok(f) => f,
    };

    println!("Running");
    let now = Instant::now();

    solution(contents);

    let execution = now.elapsed().as_secs_f64();
    print!("Execution: ");
    print_time(execution);
}

fn main() {
    let solutions: Vec<Solution> = vec![
        day01::day01,
        day02::day02,
        day03::day03,
        day04::day04,
        day05::day05,
    ];

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

    run_solution(solutions[day_number - 1], &path_str);
}

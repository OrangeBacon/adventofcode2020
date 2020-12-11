#![feature(str_split_once)]
#![feature(destructuring_assignment)]

use anyhow::{Error, Result};
use clap::clap_app;
use libaoc::{get_solution, FloatTime, Solution};
use linkme::distributed_slice;
use regex::Regex;
use std::{collections::HashMap, fs, ops::Range, time::Instant};
use strfmt::Format;

mod days;

#[distributed_slice]
pub static SOLUTIONS: [Solution] = [..];

/// gets the data to run a solution on
/// takes the index of the solution that is being run
/// if path is none, then the default data is used, otherwise reads from path
fn get_data(solution: usize, path: Option<&str>) -> Result<String> {
    if path.is_none() {
        Ok(get_solution(&*SOLUTIONS, solution)?.file.to_string())
    } else {
        Ok(fs::read_to_string(&path.unwrap())?)
    }
}

/// runs a solution with a given index and input data
/// if debug is true, then timing infomation is printed out about the
/// solution, otherwise just the answers are printed
fn run_solution(sol_index: usize, path: Option<&str>, debug: bool) {
    let sol = get_solution(&*SOLUTIONS, sol_index).expect(&format!("{}", sol_index));

    if debug {
        println!("Running Day {} - {}", sol.number, sol.name);
    } else {
        print!("Day {} {}: ", sol.number, sol.name);
    }

    let file_data = get_data(sol_index, path).unwrap();

    let now = Instant::now();
    let res = sol.run(file_data);
    let end = now.elapsed().as_secs_f64();

    match res {
        Ok(val) => {
            if debug {
                print!("{:#?}", val)
            } else {
                print!("{}", val)
            }
        }
        Err(err) => print!("{:#?}", err),
    }

    if debug {
        println!("  Total: {}", FloatTime::from(end));
    } else {
        println!("");
    }
}

/// parses a string as the number of a problem to run
/// if none passed, returns the latest day present
/// otherwise parses the string as an integer and checks it is in range
/// returns the index into the solutions array
fn parse_day_number(day: Option<&str>) -> Result<usize> {
    if day.is_none() {
        return Ok(SOLUTIONS.len());
    }

    let day = day.unwrap();
    let day_number = match day.parse::<usize>() {
        Ok(u) => u,
        Err(e) => {
            return Err(Error::msg(format!("invalid day number passed: {}", e)));
        }
    };

    if day_number > SOLUTIONS.len() {
        return Err(Error::msg(format!(
            "Day number passed doesn't exist: {}",
            day_number
        )));
    };

    if day_number == 0 {
        return Err(Error::msg("Day 0 does not exist"));
    }

    Ok(day_number)
}

/// parses a range, to be interpreted as solution numbers
/// is two numbers seperated by `..`, where both numbers are optional
/// the first number defaults to zero, the last to the most recent day
/// returns a range into the solutions array
fn parse_day_range(value: &str) -> Result<Range<usize>> {
    let range = Regex::new(r"^(\d*)..(\d*)$").unwrap();

    match range.captures(value) {
        None => Err(Error::msg(format!("\"{}\" is not a valid range", value))),
        Some(c) => {
            let start = match &c[1] {
                "" => 0,
                a => parse_day_number(Some(a))?,
            };
            let end = match &c[2] {
                "" => SOLUTIONS.len() - 1,
                a => parse_day_number(Some(a))?,
            };
            Ok(start..(end + 1))
        }
    }
}

/// runs a single day based on the command line arguments
fn run_single(matches: &clap::ArgMatches) -> Result<()> {
    let day_number = parse_day_number(matches.value_of("day"))?;

    run_solution(day_number, matches.value_of("file"), true);

    Ok(())
}

/// runs more than one day based on the command line arguments
fn run_multiple(matches: &clap::ArgMatches) -> Result<()> {
    let matches = matches.subcommand_matches("multiple").unwrap();

    // true = run this day, false = don't run
    let mut to_run = vec![false; SOLUTIONS.len()];

    let values = match matches.values_of("range") {
        Some(v) => v,
        None => {
            println!("No days specified");
            return Ok(());
        }
    };

    for value in values {
        if value.contains("..") {
            // parse range
            let range = parse_day_range(value)?;
            for i in range {
                to_run[i] = true;
            }
        } else {
            // parse single number
            let num = parse_day_number(Some(value))?;
            to_run[num] = true;
        }
    }

    let number_of_days = to_run.iter().fold(0, |a, &b| if b { a + 1 } else { a });
    println!(
        "Running {} day{}",
        number_of_days,
        if number_of_days == 1 { "" } else { "s" }
    );

    let path_str = matches.value_of("files");
    let use_default = path_str.is_none();

    // construct vector of paths, as specified by the "files" argument
    // or makes vector of none, for use the default data
    let mut paths = vec![];
    for i in 0..SOLUTIONS.len() {
        paths.push(if use_default {
            None
        } else {
            let mut args = HashMap::new();
            args.insert("day".to_owned(), i);
            Some(path_str.unwrap().format(&args)?)
        })
    }

    let now = Instant::now();

    // run all specified days
    for (i, &day) in to_run.iter().enumerate() {
        if day {
            run_solution(i + 1, paths[i].as_deref(), false);
        }
    }

    println!(
        "Total time: {}",
        FloatTime::from(now.elapsed().as_secs_f64())
    );

    Ok(())
}

fn main() -> Result<()> {
    let now = Instant::now();

    let matches = clap_app!(aoc2020 =>
        (version: "8.0")
        (author: "Ella M. @OrangeBacon#0273")
        (about: "Advent of Code 2020 solutions")
        (@arg day: +takes_value "Which day's code to run, default: latest day")
        (@arg file: -f --file +takes_value "Alternative test input, default: my test input files")
        (@subcommand multiple =>
            (version: "1.0")
            (about: "Run more than one included solution")
            (@arg range: ... +takes_value "range of days to run, inclusive, either end optional")
            (@arg files: -f --file +takes_value "Pattern to use for finding the inputs, replaces {day} with day number, default: my inputs")
        )
    ).get_matches();

    let result = match matches.subcommand_name() {
        Some("multiple") => run_multiple(&matches),
        _ => run_single(&matches),
    };

    let now = now.elapsed().as_secs_f64();
    println!("Whole program: {}", FloatTime::from(now));
    result
}

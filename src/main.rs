#![feature(or_patterns)]
#![feature(str_split_once)]
#![feature(destructuring_assignment)]

use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use libaoc::{AocFile, FloatTime, Solution, Timer};
use linkme::distributed_slice;
use regex::Regex;
use std::{error::Error, fmt, time::Instant};

/// not actually used, is just to tell the compiler to compile the days so that
/// distributed slice can work properly
mod days;

/// the array of all code avaliable to run
#[distributed_slice]
pub static SOLUTIONS: [Solution] = [..];

#[distributed_slice]
pub static FILES: [AocFile] = [..];

/// custom error type for the command line interface
/// only exists so that multiple errors can be combined, so all errors
/// are reported, not just the first
#[derive(Debug)]
enum CliError {
    InvalidDayNumber(String, Box<dyn Error>),
    NoDayZero,
    DayOutOfRange(usize),
    InvalidRange(String),
    BadArgument(String),
    MultipleError(Vec<CliError>),
}

impl CliError {
    /// converts two errors into one error representing both of the inputs
    /// is the main reason that the error type exists
    fn from(e1: CliError, e2: CliError) -> CliError {
        use CliError::*;

        match (e1, e2) {
            (MultipleError(v1), MultipleError(v2)) => {
                let mut res = vec![];
                res.extend(v1);
                res.extend(v2);
                MultipleError(res)
            }
            (MultipleError(v), e) | (e, MultipleError(v)) => {
                let mut res = vec![];
                res.extend(v);
                res.push(e);
                MultipleError(res)
            }
            (e1, e2) => MultipleError(vec![e1, e2]),
        }
    }
}

impl Error for CliError {}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use CliError::*;

        match self {
            InvalidDayNumber(num, err) => {
                write!(f, "- Invalid day number passed: `{}` - {}", num, err)
            }
            NoDayZero => writeln!(f, "- Day 0 does not exist"),
            DayOutOfRange(num) => writeln!(
                f,
                "- Day {} is too large, maximum is {}",
                num,
                Solution::latest_day(&SOLUTIONS)
            ),
            InvalidRange(e) => writeln!(f, "- Invalid range `{}`", e),
            BadArgument(e) => writeln!(f, "- Error: Could not find anything to run matching {}", e),
            MultipleError(e) => {
                for error in e {
                    write!(f, "{}", error)?;
                }
                Ok(())
            }
        }
    }
}

/// runs a solution with a given index and input data
/// if debug is true, then timing infomation is printed out about the
/// solution, otherwise just the answers are printed
fn run_solution(solution: &Solution, debug: bool) {
    if debug {
        println!("Running Day {} - {}\n", solution.number, solution.name);
    } else {
        print!("Day {} {}: ", solution.number, solution.name);
    }

    let file_data = AocFile::get(&*FILES, solution.number).unwrap();

    let mut timer = Timer::new();
    let res = solution.run(&mut timer, file_data);
    timer.stop();

    match res {
        Ok(val) => {
            if debug {
                println!("{:#?}", val)
            } else {
                println!("{}", val)
            }
        }
        Err(err) => println!("{:#?}", err),
    }

    if debug {
        println!("{:?}", timer);
    }
}

/// parses a string as the number of a problem to run
/// if none passed, returns the latest day present
/// otherwise parses the string as an integer and checks it is in range
/// returns the index into the solutions array
/// if not a valid range, returns err, if valid range but numbers out of range
/// returns ok(err)
fn parse_day_number(day: Option<&str>) -> Result<Result<usize, CliError>, CliError> {
    if day.is_none() {
        return Ok(Ok(SOLUTIONS.len()));
    }

    let day = day.unwrap();
    let day_number = match day.parse::<usize>() {
        Ok(u) => u,
        Err(e) => {
            return Err(CliError::InvalidDayNumber(day.to_string(), Box::new(e)));
        }
    };

    if day_number > SOLUTIONS.len() {
        return Ok(Err(CliError::DayOutOfRange(day_number)));
    };

    if day_number == 0 {
        return Ok(Err(CliError::NoDayZero));
    }

    Ok(Ok(day_number))
}

/// the type representing everything that is valid to pass on the command line
/// digit = single day number
/// range = range of day numbers, inclusive both ends
/// name = class of solutions, eg solve, visualise, bench
/// file = a file name pattern, passed using -f flag to distinguish it from name
#[derive(Clone, Copy, Debug)]
enum ArgumentType<'a> {
    Digit(usize),
    Range(usize, usize),
    Name(&'a str),
    File(&'a str),
}

impl<'a> fmt::Display for ArgumentType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use ArgumentType::*;

        match self {
            Digit(a) => write!(f, "{}", a)?,
            Range(a, b) => write!(f, "{}..{}", a, b)?,
            Name(a) | File(a) => write!(f, "\"{}\"", a)?,
        }

        Ok(())
    }
}

/// a single set of solutions to run
/// the results of combining the command line arguments
#[derive(Clone, Default, Debug)]
struct ProcessedArgument<'a> {
    name: Option<&'a str>,
    number: Option<ArgumentType<'a>>,
    file_pattern: Option<&'a str>,
}

impl<'a> fmt::Display for ProcessedArgument<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let (Some(name), Some(number)) = (self.name, self.number) {
            write!(f, "{}({})", name, number)
        } else {
            Ok(())
        }
    }
}

/// converts between the command line input and the solutions it represents
/// returns all solutions in the range provided if applicable
/// if no solutions found returns None, so vec.len() is always > 0
/// if some returned
fn get_solutions(argument: &ProcessedArgument) -> Option<Vec<&'static Solution>> {
    use ArgumentType::*;

    let name = argument.name?;
    let day_range = argument.number?;
    let day_range = match day_range {
        Digit(a) => a..=a,
        Range(a, b) => a..=b,
        _ => return None,
    };

    let mut solutions = vec![];
    for day in day_range {
        if let Ok(sol) = Solution::get(&SOLUTIONS, day, name) {
            solutions.push(sol);
        }
    }

    if solutions.is_empty() {
        None
    } else {
        Some(solutions)
    }
}

/// parses a range, to be interpreted as solution numbers
/// is two numbers seperated by `..`, where both numbers are optional
/// the first number defaults to zero, the last to the most recent day
/// returns a range into the solutions array
/// if not a range returns err, if is a range but the numbers provided
/// are out of range, returns ok(err)
fn parse_day_range(value: &str) -> Result<Result<ArgumentType, CliError>, CliError> {
    let range = Regex::new(r"^(\d*)..(\d*)$").unwrap();

    match range.captures(value) {
        None => Err(CliError::InvalidRange(range.to_string())),
        Some(c) => {
            let start = match &c[1] {
                "" => Ok(0),
                a => parse_day_number(Some(a))?,
            };
            let end = match &c[2] {
                "" => Ok(Solution::latest_day(&SOLUTIONS)),
                a => parse_day_number(Some(a))?,
            };

            match (start, end) {
                (Err(e1), Err(e2)) => Ok(Err(CliError::from(e1, e2))),
                (Err(e), _) | (_, Err(e)) => Ok(Err(e)),
                (Ok(a), Ok(b)) => Ok(Ok(ArgumentType::Range(a, b))),
            }
        }
    }
}

/// takes the command line arguments from clap, gets the solutions that they
/// represent and runs them.  Is essentially the main function
fn interpret_arguments(arguments: ArgMatches) -> Result<(), CliError> {
    // the most recent day number, used in default values
    let latest_day = ArgumentType::Digit(Solution::latest_day(&SOLUTIONS));

    // list of day numbers if none provided
    let default = vec![(0, latest_day)];

    let solutions = if let Some(values) = arguments.values_of("day") {
        // as the values exist, the indicies must also exist
        let indicies = arguments.indices_of("day").unwrap();

        // the indicies are included so the -f flag inputs can be sorted into
        // an array with the day numbers correctly
        let values = values.zip(indicies).map(|(arg, index)| {
            if let Ok(num) = parse_day_number(Some(arg)) {
                match num {
                    Ok(a) => Ok((index, ArgumentType::Digit(a))),
                    Err(e) => Err(e),
                }
            } else if let Ok(arg) = parse_day_range(arg) {
                match arg {
                    Ok(a) => Ok((index, a)),
                    Err(e) => Err(e),
                }
            } else {
                Ok((index, ArgumentType::Name(arg)))
            }
        });

        // collect all the errors together so they can all be returned
        // should allow for better cli ux
        let mut errors = vec![];
        let mut results = vec![];
        for value in values {
            match value {
                Ok(e) => results.push(e),
                Err(e) => errors.push(e),
            }
        }
        if !errors.is_empty() {
            return Err(CliError::MultipleError(errors));
        }
        results
    } else {
        default
    };

    // get the files passed, if any
    let file_paths = if let Some(paths) = arguments.values_of("file") {
        let indicies = arguments.indices_of("file").unwrap();
        paths
            .zip(indicies)
            .map(|(arg, index)| (index, ArgumentType::File(arg)))
            .collect()
    } else {
        vec![]
    };

    // combine the files and day numbers into single array
    let mut solutions: Vec<_> = solutions.iter().chain(file_paths.iter()).collect();

    // this sort is the reason the indicies are required, they are ignored after
    // this point
    solutions.sort_unstable_by(|&(a, _), (b, _)| a.cmp(b));

    // combine the arguments passed into groups
    // if the item provided has already been set for the latest group
    // creates a new group and sets it in there
    let mut processed_arguments = vec![ProcessedArgument::default()];
    for (_, argument) in solutions {
        use ArgumentType::*;

        let current_index = processed_arguments.len() - 1;
        let mut current_argument = &mut processed_arguments[current_index];

        match argument {
            File(a) => {
                if current_argument.file_pattern.is_none() {
                    current_argument.file_pattern = Some(a)
                } else {
                    processed_arguments.push(ProcessedArgument {
                        file_pattern: Some(a),
                        ..Default::default()
                    });
                }
            }
            a @ (Digit(_) | Range(_, _)) => {
                if current_argument.number.is_none() {
                    current_argument.number = Some(*a)
                } else {
                    processed_arguments.push(ProcessedArgument {
                        number: Some(*a),
                        ..Default::default()
                    });
                }
            }
            Name(a) => {
                if current_argument.name.is_none() {
                    current_argument.name = Some(a)
                } else {
                    processed_arguments.push(ProcessedArgument {
                        name: Some(a),
                        ..Default::default()
                    });
                }
            }
        }
    }

    // find the solutions from the arguments provided
    // for now ignores file paths, todo: not this
    let mut solutions = vec![];
    let mut has_failed = false;
    // default value, never read, needed to get it to compile as cannot tell
    // this will not be read
    let mut err = CliError::NoDayZero;
    for argument in &processed_arguments {
        let mut argument = argument.clone();
        if argument.name.is_none() {
            argument.name = Some("solve")
        }
        if argument.number.is_none() {
            argument.number = Some(latest_day)
        }
        if argument.file_pattern.is_none() {
            argument.file_pattern = if argument.name == Some("solve") {
                None
            } else {
                Some("{name}/day{day}.{type}")
            }
        }

        let sols = get_solutions(&argument);
        if let Some(sols) = sols {
            solutions.extend(sols);
        } else if !has_failed {
            has_failed = true;
            err = CliError::BadArgument(format!("{}", argument));
        } else {
            err = CliError::from(err, CliError::BadArgument(format!("{}", argument)));
        }
    }

    // return error if failed to find solutions for one of the processed arguments
    if has_failed {
        return Err(err);
    }

    // de duplicate the solutions, for proper de-duplication requires sorting
    // first, anyway is nicer to run in sorted order
    solutions.sort_unstable_by(|a, b| a.number.cmp(&b.number));
    let found_count = solutions.len();
    solutions.dedup();
    if solutions.len() != found_count {
        eprintln!(
            "Warning: removed {} duplicate(s) passed on command line",
            found_count - solutions.len()
        );
    }

    // run all the solutions
    for solution in &solutions {
        run_solution(solution, solutions.len() == 1);
    }

    Ok(())
}

fn main() {
    let now = Instant::now();

    let matches = App::new("aoc2020")
        .author("Ella M. @OrangeBacon#0273")
        .about("Advent of Code 2020 solutions")
        .version(&*format!(
            "{}, upto day {}",
            env!("VERGEN_SHA_SHORT"),
            Solution::latest_day(&SOLUTIONS)
        ))
        .arg(
            Arg::with_name("day")
                .help("The portions of this program to run")
                .multiple(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .multiple(true)
                .number_of_values(1)
                .help("The file path pattern to use"),
        )
        .get_matches();

    let result = interpret_arguments(matches);
    if let Err(msg) = result {
        eprintln!("Error(s) occured: \n{}Exiting.", msg);
        return;
    }

    let now = now.elapsed().as_secs_f64();
    println!("Whole program: {}", FloatTime::from(now));
}

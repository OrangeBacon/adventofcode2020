use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// How this project is built:
/// - detects all files in the src/days folder
/// - iterates through each file
/// - for every function annotated with #[aoc] add it to a vec
/// - import that file using `mod {file name}`
/// - create a constant global array with all of the runners in
/// - export the global array
fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(&out_dir).join("solutions.rs");

    let mut build_path = PathBuf::from(file!());
    build_path.pop();
    build_path.push("src");
    build_path.push("days");

    let mut files = fs::read_dir(build_path)?
        .map(|x| x.map(|e| fs::canonicalize(e.path()).unwrap()))
        .collect::<Result<Vec<_>, _>>()?;
    files.sort();
    let days = files
        .iter()
        .map(|x| (x, fs::read_to_string(x).unwrap()))
        .collect::<Vec<_>>();
    let days = days
        .iter()
        .map(|(path, x)| (path, x.lines().collect::<Vec<_>>()));

    let mut solutions = vec![];
    for (path, day) in days {
        for window in day.windows(2) {
            let first = window[0];
            let second = window[1];
            if first.starts_with("#[aoc") {
                let name = second.split_whitespace().nth(2).unwrap();
                let comp = path.file_stem().unwrap().to_str().unwrap().to_string();
                solutions.push((
                    format!("#[path={:?}]mod {};", path, comp),
                    format!("{}::{}", comp, name.split('(').next().unwrap()),
                ));
            }
        }
    }

    let (r#mod, r#use): (Vec<_>, Vec<_>) = solutions.iter().cloned().unzip();

    let r#mod = r#mod.join("\n");
    let r#use = r#use.join(",");

    let output = format!(
        "use libaoc::SolutionGetter;

{}

pub const SOLUTIONS: &[SolutionGetter] = &[{}];",
        r#mod, r#use
    );

    fs::write(&dest_path, output)?;

    Ok(())
}

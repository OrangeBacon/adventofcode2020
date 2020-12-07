fn iter(lines: &Vec<Vec<char>>, depth: usize, height: usize) -> i32 {
    let mut x = 0;
    let mut trees = 0;
    for (i, line) in lines.iter().enumerate() {
        if i % height != 0 {
            continue;
        }
        if line[x] == '#' {
            trees += 1;
        }
        x = (x + depth) % line.len();
    }
    trees
}

pub fn day03(input: String) {
    let lines = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let three = iter(&lines, 3, 1);
    println!("{}", three);

    println!(
        "{}",
        iter(&lines, 1, 1) * three * iter(&lines, 5, 1) * iter(&lines, 7, 1) * iter(&lines, 1, 2)
    );
}

use regex::Regex;

pub fn day02(input: String) {
    let lines: Vec<_> = input.lines().collect();

    let extract = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();

    let mut count_1 = 0;
    let mut count_2 = 0;
    for line in &lines {
        let matches = extract.captures(line).unwrap();
        let min = matches.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let max = matches.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let c = matches.get(3).unwrap().as_str().chars().next().unwrap();
        let password = matches.get(4).unwrap().as_str();

        let c_count = password.matches(c).count();
        if c_count >= min && c_count <= max {
            count_1 += 1;
        }

        let min_c = password.chars().nth(min-1).unwrap();
        let max_c = password.chars().nth(max-1).unwrap();
        if  (min_c == c && max_c != c) || (min_c != c && max_c == c) {
            count_2 += 1;
        }
    }

    println!("part 1: {}", count_1);
    println!("part 2: {}", count_2);
}
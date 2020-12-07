use regex::Regex;

pub fn day04(input: String) {
    let datas: Vec<Vec<Vec<_>>> = input
        .split("\n\n")
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.split(':').collect())
                .collect()
        })
        .collect();

    let res: Vec<_> = datas
        .iter()
        .filter(|x| x.len() == 8 || (x.len() == 7 && x.iter().find(|y| y[0] == "cid").is_none()))
        .collect();

    println!("{:?}", res.len());

    let birth = Regex::new(r"19[2-9][0-9]|200[0-2]").unwrap();
    let issue = Regex::new(r"201[0-9]|2020").unwrap();
    let expire = Regex::new(r"202[0-9]|2030").unwrap();
    let height = Regex::new(r"^(?:1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)$").unwrap();
    let hair = Regex::new(r"#[0-9a-f]{6}").unwrap();
    let eye = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    let pid = Regex::new(r"^[0-9]{9}$").unwrap();
    let cid = Regex::new("").unwrap();

    let mut valid = 0;
    for pass in res {
        let mut passed = true;
        for sec in pass {
            if !match sec[0] {
                "byr" => &birth,
                "iyr" => &issue,
                "eyr" => &expire,
                "hgt" => &height,
                "hcl" => &hair,
                "ecl" => &eye,
                "pid" => &pid,
                "cid" => &cid,
                _ => panic!(),
            }
            .is_match(sec[1])
            {
                passed = false;
                break;
            }
        }
        if passed {
            valid += 1;
        }
    }

    println!("{}", valid);
}

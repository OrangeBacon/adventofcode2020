use regex::Regex;

pub fn day04(input: String) {
    let datas : Vec<Vec<Vec<_>>> = input.split("\n\n").map(|x| x.split_whitespace().map(|y|y.split(':').collect()).collect()).collect();

    let res : Vec<_> = datas.iter()
        .filter(|x| x.len() == 8 || (x.len() == 7 && x.iter().find(|y| y[0]=="cid").is_none()))
        .collect();

    println!("{:?}", res.len());

    let height = Regex::new(r"(\d+)(cm|in)").unwrap();
    let hair = Regex::new(r"#[0-9a-f]{6}").unwrap();
    let eye = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    let pid = Regex::new(r"^[0-9]{9}$").unwrap();

    let mut valid = 0;
    for pass in res {
        let mut passed = true;
        for sec in pass {
            let suc = match sec[0] {
                "byr" => {
                    let num = sec[1].parse::<i32>();
                    if let Ok(n) = num {
                        n >= 1920 && n <= 2002
                    } else {
                        false
                    }
                }
                "iyr" => {
                    let num = sec[1].parse::<i32>();
                    if let Ok(n) = num {
                        n >= 2010 && n <= 2020
                    } else {
                        false
                    }
                }
                "eyr" => {
                    let num = sec[1].parse::<i32>();
                    if let Ok(n) = num {
                        n >= 2020 && n <= 2030
                    } else {
                        false
                    }
                }
                "hgt" => {
                    let test = height.captures(sec[1]);
                    if let Some(cap) = test {
                        let num = cap.get(1).unwrap().as_str().parse::<i32>();
                        if let Ok(n) = num {
                            if cap.get(2).unwrap().as_str() == "cm" {
                                n >= 150 && n <= 193
                            } else {
                                n >= 59 && n <= 76
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                "hcl" => hair.is_match(sec[1]),
                "ecl" => eye.is_match(sec[1]),
                "pid" => pid.is_match(sec[1]),
                "cid" => true,
                _ => panic!()
            };
            if !suc {passed = false; break;}
        }
        if passed {valid += 1;}
    }

    println!("{}", valid);
}
use std::cmp::{max};

pub fn day05(input: String) {
    let lines : Vec<_> = input.lines().collect();

    let mut seats = [[false; 8]; 128];

    let highest = lines.iter().fold(0, |acc, &line| {
        let mut row = 0;
        let mut col = 0;
        let mut row_size = 64;
        let mut col_size = 4;
        for c in line.chars() {
            match c {
                'F' => row_size /= 2,
                'B' => { // upper
                    row += row_size;
                    row_size /= 2;
                }
                'R' => { // upper
                    col += col_size;
                    col_size /= 2;
                }
                'L' => col_size /= 2,
                _ => panic!()
            }
        }
        seats[row][col] = true;
        max(acc, row * 8 + col)
    });

    println!("{}", highest);

    let mut found = false;
    for (y, row) in seats.iter().enumerate() {
        if !found {
            found = row.iter().fold(false, |a,x|a|x);
        } else {
            let res = row.iter().enumerate().fold(0, |_,(x,v)|if *v{0} else {y*8+x});
            if res > 0 {
                println!("{}", res);
                break;
            }
        }
    }

    day05v2(input);
}

use std::collections::HashSet;

fn day05v2(input: String) {
    let lines : HashSet<_> = input.lines().map(|v|
        i32::from_str_radix(&v.replace("F","0").replace("B", "1").replace("L", "0").replace("R", "1"), 2)
        .unwrap()
    ).collect();
    println!("{}", lines.iter().max().unwrap());

    let min = *lines.iter().min().unwrap();
    let max = *lines.iter().max().unwrap();
    let ids : HashSet <_> = (min..max).collect();
    println!("{}", (&ids - &lines).iter().next().unwrap());
}
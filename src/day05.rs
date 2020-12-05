use std::cmp::{max};

pub fn day05(input: String) {
    let lines : Vec<_> = input.lines().collect();

    let mut highest = 0;

    let mut seats = [[(false, 0); 8]; 128];

    for line in lines {
        let mut row = 0;
        let mut col = 0;
        let mut row_size = 128;
        let mut col_size = 8;
        for c in line.chars() {
            match c {
                'F' => { // lower
                    row_size /= 2;
                }
                'B' => { // upper
                    row += row_size;
                    row_size /= 2;
                }
                'R' => { // upper
                    col += col_size;
                    col_size /= 2;
                }
                'L' => { // lower
                    col_size /= 2;
                }
                _ => panic!()
            }
        }
        seats[row/2][col/2] = (true, row/2 * 8 + col/2);
        highest = max(highest, row/2 * 8 + col/2);
    }

    let mut found = false;
    let mut id = 0;
    'out: for (y, row) in seats.iter().enumerate() {
        if !found {
            for seat in row.iter() {
                if (*seat).0 {
                    found = true;
                }
            }
        } else {
            for (x, seat) in row.iter().enumerate() {
                if !(*seat).0 {
                    id = y * 8 + x;
                    break 'out;
                }
            }
        }
    }

    println!("{}", highest);
    println!("{}", id);
}
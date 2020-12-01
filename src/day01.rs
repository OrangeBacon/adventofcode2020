pub fn day01(input: String) {
    let nums: Vec<_> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    'out: for (i, num) in nums.iter().enumerate() {
        for second in nums[(i+1)..].iter() {
            if num + second == 2020 {
                println!("{} * {} = {}", num, second, num * second);
                break 'out;
            }
        }
    }

    for (i, num) in nums.iter().enumerate() {
        for (j, second) in nums[(i+1)..].iter().enumerate() {
            for third in nums[(j+1)..].iter() {
                if num + second + third == 2020 {
                    println!("{} * {} * {} = {}", num, second, third, num * second * third);
                    return;
                }
            }
        }
    }
}
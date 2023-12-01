use std::fs::read_to_string;

pub fn d01() {
    let file = read_to_string("../inputs/input01.txt").unwrap();
    let lines = file.lines();

    let mut sum: i32 = 0;

    for line in lines {
        let mut s = line.to_string();
        s.retain(|c| c.is_ascii_digit());
        let mut numbers = s.chars().nth(0).unwrap().to_string();
        numbers.push(s.chars().last().unwrap());
        println!("line : {}", numbers);
        sum += numbers.parse::<i32>().unwrap();
    }

    println!("{}", sum.to_string());
}
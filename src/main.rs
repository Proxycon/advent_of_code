use std::{cmp, fs::read_to_string};

fn main() {
    d01("inputs/input1.txt")
}

fn d01(input: &str) {
    let file = read_to_string(input).unwrap();
    let mut lines = file.lines();

    let mut max = 0;
    let mut current = 0;

    while let Some(line) = lines.next() {
        if line.len() == 0 {
            max = cmp::max(max, current);
            current = 0;
        } else {
            let ration = line.parse::<i32>().unwrap();
            current += ration;
        }
        println!("max: {}, current: {}", max, current)
    }
    max = cmp::max(max, current);
    println!("Maximum Calories ccarried: {}", max)
}

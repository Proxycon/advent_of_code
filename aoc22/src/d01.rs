use std::{cmp, fs::read_to_string};

pub fn d01(input: &str) {
    let file = read_to_string(input).unwrap();
    let lines = file.lines();

    let mut max = 0;
    let mut current = 0;

    for line in lines {
        if line.is_empty() {
            max = cmp::max(max, current);
            current = 0;
        } else {
            let ration = line.parse::<i32>().unwrap();
            current += ration;
        }
    }
    max = cmp::max(max, current);
    println!("#d01 - 1\nMaximum Calories carried: {}", max)
}

pub fn d01_2(input: &str) {
    let file = read_to_string(input).unwrap();
    let lines = file.lines();

    let mut elfs: Vec<u32> = Vec::new();
    let mut current: u32 = 0;

    for line in lines {
        if line.is_empty() {
            elfs.push(current);
            current = 0;
        } else {
            let ration = line.parse::<u32>().unwrap();
            current += ration;
        }
    }
    elfs.sort();
    elfs.reverse();
    println!("#d01 - 2\nelfs: {:#?}", vec![elfs[0], elfs[1], elfs[2]]);
    println!(
        "calories carried by top 3 elfs: {:#?}",
        elfs[..3].iter().sum::<u32>()
    )
}

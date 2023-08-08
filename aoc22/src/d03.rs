use std::{fs::read_to_string, collections::HashSet};

pub fn d03_01 (input: &str) {
    let file = read_to_string(input).unwrap();
    let lines = file.lines();

    let mut prio_sum = 0;

    for line in lines {
        let (left, right) = line.split_at(line.len()/2);
        let left: HashSet<char> = left.chars().collect();
        let right: HashSet<char> = right.chars().collect();

        let union = get_single_intersect(left, right);
        let prio = get_prio(union);
        prio_sum += prio;
    }
    println!("#d03:");
    println!("prio sum: {}", prio_sum);
}

pub fn d03_02(input: &str){
    let file = read_to_string(input).unwrap();
    let lines = file.lines();

    let mut prio_sum = 0;
    let mut set: HashSet<char> = HashSet::new();

    for (i, line) in lines.enumerate() {
        // println!(" group counter: {}", i % 3);
        if i % 3 == 0 { // after each group
            // println!("{:#?}", set);
            if !set.is_empty(){
                assert!(set.len() == 1);
                prio_sum += get_prio(*set.iter().last().unwrap())
            }
            set = line.chars().collect();
        } else {
            // let tmp_set: HashSet<char> = line.chars().collect();
            set.retain(|e| line.contains(*e));
        }     
    }
    assert!(set.len() == 1);
    prio_sum += get_prio(*set.iter().last().unwrap());
    println!("#d03 - 2:");
    println!("badge prio sum: {}", prio_sum);

}

fn get_prio(c:char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - 96
    } else if c.is_uppercase() {
        (c as u32) - 64 + 26
    } else {
        0
    }
}

fn get_single_intersect(left: HashSet<char>, right: HashSet<char>) -> char {
    let intersect = left.intersection(&right).collect::<String>();
    // println!("union of {:#?} and {:#?} is {}",&left, &right, union);
    intersect.chars().last().unwrap()
}
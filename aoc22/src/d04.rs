use std::fs::read_to_string;

enum Condition {
    Overlap,
    Contain
}

pub fn d04_01(input: &str){
    let file = read_to_string(input).unwrap();
    let lines = file.lines();

    let n = lines.filter(|e| overlap(e, Condition::Contain)).count();
    println!("d03 - 01");
    println!("containing assignements: {}", n);

}

pub fn d04_02(input: &str){
    let file = read_to_string(input).unwrap();
    let lines = file.lines();

    let n = lines.filter(|e| overlap(e, Condition::Overlap)).count();
    println!("d03 - 01");
    println!("overlapping assignements: {}", n);

}

fn overlap(line: &&str, c: Condition) -> bool {
    let Some((first, last)) = line.split_once(',') else { return false };
    let Some((x1, x2)) = first.split_once('-') else { return false };
    let Some((y1, y2)) = last.split_once('-') else { return false };
    //println!("containment of {}: {}", line, containing(x1.trim().parse().unwrap(), x2.trim().parse().unwrap(), y1.trim().parse().unwrap(), y2.trim().parse().unwrap()));
    match c {
        Condition::Contain => containing(x1.trim().parse().unwrap(), x2.trim().parse().unwrap(), y1.trim().parse().unwrap(), y2.trim().parse().unwrap()),
        Condition::Overlap => overlapping(x1.trim().parse().unwrap(), x2.trim().parse().unwrap(), y1.trim().parse().unwrap(), y2.trim().parse().unwrap()),
    }
    
}

fn containing(x1: u32, x2: u32, y1: u32, y2 :u32) -> bool {
    (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2)
}

fn overlapping(x1: u32, x2: u32, y1: u32, y2 :u32) -> bool{
    x1 <= y2 && y1 <= x2
}

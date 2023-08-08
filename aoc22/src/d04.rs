use std::fs::read_to_string;


pub fn d04_01(input: &str){
    let file = read_to_string(input).unwrap();
    let lines = file.lines();

    let n = lines.filter(overlap).count();
    println!("d03 - 01");
    println!("containing assignements: {}", n);

}

fn overlap(line: &&str) -> bool {
    let Some((first, last)) = line.split_once(',') else { return false };
    let Some((x1, x2)) = first.split_once('-') else { return false };
    let Some((y1, y2)) = last.split_once('-') else { return false };
    //println!("containment of {}: {}", line, containing(x1.trim().parse().unwrap(), x2.trim().parse().unwrap(), y1.trim().parse().unwrap(), y2.trim().parse().unwrap()));
    containing(x1.trim().parse().unwrap(), x2.trim().parse().unwrap(), y1.trim().parse().unwrap(), y2.trim().parse().unwrap())
}

fn containing(x1: u32, x2: u32, y1: u32, y2 :u32) -> bool {
    (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2)
}

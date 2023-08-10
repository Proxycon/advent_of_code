use std::{fs::read_to_string, collections::{VecDeque, HashSet}};


pub fn d06_01(input:&str){
    let file = read_to_string(input).unwrap();
    let mut buff: VecDeque<char> = VecDeque::new();
    for (i, c) in file.chars().enumerate(){
        buff.push_back(c);
        if buff.len() < 4{continue;} else if is_unique(&mut buff) {
            println!("d06");
            println!("first unique 4-sequence {} at index {}", buff.iter().collect::<String>(), i+1);
            break;
        }
        buff.pop_front();
    }
}

pub fn d06_02(input:&str){
    let file = read_to_string(input).unwrap();
    let mut buff: VecDeque<char> = VecDeque::new();
    for (i, c) in file.chars().enumerate(){
        buff.push_back(c);
        if buff.len() < 14{continue;} else if is_unique(&mut buff) {
            println!("d06 - 02");
            println!("first unique 14-sequence {} at index {}", buff.iter().collect::<String>(), i+1);
            break;
        }
        buff.pop_front();
    }
}

fn is_unique(v: &mut VecDeque<char>) -> bool{
    let mut set: HashSet<char> = HashSet::new();
    for c in v{
        if !set.insert(*c){return false}
    }
    true
}
use std::cmp::max;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::vec;

fn create_stacks() -> HashMap<&'static str, Vec<char>> {
    let mut stacks: HashMap<&str, Vec<char>> = HashMap::new();
    stacks.insert("1", vec!['Q', 'M', 'G', 'C', 'L']);
    stacks.insert("2", vec!['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G']);
    stacks.insert("3", vec!['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R']);
    stacks.insert("4", vec!['J', 'F', 'D', 'V', 'Q', 'P']);
    stacks.insert("5", vec!['N', 'F', 'M', 'S', 'L', 'B', 'T']);
    stacks.insert("6", vec!['R', 'N', 'V', 'H', 'C', 'D', 'P']);
    stacks.insert("7", vec!['H', 'C', 'T']);
    stacks.insert("8", vec!['G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P']);
    stacks.insert("9", vec!['Z', 'F', 'H', 'G']);

    stacks
}

pub fn d05_01(input: &str) {
    let mut stacks = create_stacks();
    let file = read_to_string(input).unwrap();
    let lines = file.lines();
    print_stacks(&stacks);
    for line in lines {
        let l = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let n: usize = l[1].parse().expect("could not parse");
        let from = l[3];
        let to = l[5];
        move_crates(&mut stacks, from, to, n);
        //let status = read_top_crates(&stacks);
        //println!("after {} status is {}", line, status);
    }
    print_stacks(&stacks);
    println!("#d05");
    println!("top crates: {}", read_top_crates(&stacks));
}

pub fn d05_02(input: &str) {
    let mut stacks = create_stacks();
    let file = read_to_string(input).unwrap();
    let lines = file.lines();

    for line in lines {
        let l = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let n: usize = l[1].parse().expect("could not parse");
        let from = l[3];
        let to = l[5];
        move_multiple_crates(&mut stacks, from, to, n);
        //let status = read_top_crates(&stacks);
        //println!("after {} status is {}", line, status);
    }
    print_stacks(&stacks);
    println!("#d05 - 02");
    println!("top crates: {}", read_top_crates(&stacks));
}

pub fn _test() {
    let mut stacks: HashMap<&str, Vec<char>> = HashMap::new();
    stacks.insert("1", vec!['A', 'C']);
    stacks.insert("2", vec!['B']);
    stacks.insert("3", vec![]);
    assert_eq!(read_top_crates(&stacks), "CB?".to_string());

    move_crates(&mut stacks, "1", "2", 1);
    print_stacks(&stacks);
    assert_eq!(read_top_crates(&stacks), "AC?".to_string());

    move_crates(&mut stacks, "2", "1", 2);
    println!("{:#?}", stacks);
    assert_eq!(read_top_crates(&stacks), "B??".to_string());

    move_crates(&mut stacks, "1", "2", 1);
    println!("{:#?}", stacks);
    assert_eq!(read_top_crates(&stacks), "CB?".to_string());

    move_crates(&mut stacks, "1", "3", 1);
    println!("{:#?}", stacks);
    assert_eq!(read_top_crates(&stacks), "ABC".to_string());
}

fn move_crates(stacks: &mut HashMap<&'static str, Vec<char>>, from: &str, to: &str, n: usize) {
    for _ in 0..n {
        if let Some(c) = stacks.get_mut(from).unwrap().pop() {
            stacks.get_mut(to).unwrap().push(c);
        }
    }
}

fn move_multiple_crates(
    stacks: &mut HashMap<&'static str, Vec<char>>,
    from: &str,
    to: &str,
    n: usize,
) {
    let at = stacks.get_mut(from).unwrap().len() - n;
    let tail = &mut stacks
        .get_mut(from)
        .unwrap()
        .drain(at..)
        .collect::<Vec<char>>();
    stacks.get_mut(to).unwrap().append(tail);
}

fn read_top_crates(stacks: &HashMap<&'static str, Vec<char>>) -> String {
    let mut res = String::from("");
    for i in 1..=stacks.len() {
        let c = stacks
            .get(i.to_string().as_str())
            .expect("stack not found")
            .last()
            .unwrap_or(&'?');
        res.push(*c);
    }
    res
}

fn print_stacks(stacks: &HashMap<&'static str, Vec<char>>) {
    let mut lines: Vec<String> = vec![];
    let mut start_line = "".to_string();
    let mut max_len = 0;
    for i in 1..=stacks.len() {
        start_line.push_str(format!(" {}  ", i).as_str());
        max_len = max(
            max_len,
            stacks
                .get(i.to_string().as_str())
                .expect("stack not found")
                .len(),
        )
    }
    lines.push(start_line);
    for i in 0..max_len {
        let mut line = "".to_string();
        for j in 1..=stacks.len() {
            line.push_str(
                format!(
                    "[{}] ",
                    stacks
                        .get(j.to_string().as_str())
                        .unwrap_or_else(|| panic!("stack {} not found", j))
                        .get(i)
                        .unwrap_or(&' ')
                )
                .as_str(),
            );
        }
        lines.push(line);
    }
    lines.reverse();
    let s: String = lines.iter().map(|line| line.to_owned() + "\n").collect();
    println!("{}", s);
}

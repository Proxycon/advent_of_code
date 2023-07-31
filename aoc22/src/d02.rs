use std::fs::read_to_string;

pub fn d02(input: &str){
    let file = read_to_string(input).unwrap();
    let lines = file.lines();
    let mut points = 0;
    for line in lines{
        let moves: Vec<&str> = line.split_whitespace().collect();
        points += evaluate_match1(moves[0], moves[1])
    }
    println!("#d02 - 1\nexpected point in standard strategy: {}", points)
}

pub fn d02_2(input: &str){
    let file = read_to_string(input).unwrap();
    let lines = file.lines();
    let mut points = 0;
    for line in lines{
        let moves: Vec<&str> = line.split_whitespace().collect();
        points += evaluate_match2(moves[0], moves[1])
    }
    println!("#d02 - 2\nexpected point in ultimate strategy: {}", points)
}

fn evaluate_match1(op:&str, you: &str) -> i32 {
    let base_points = get_base_points(you);

    let match_points = get_match_points_from_moves(op, you);

    base_points + match_points
}

fn evaluate_match2(op:&str, goal: &str) -> i32 {
    let you = get_move(op, goal);
    let base_points = get_base_points(you);

    let match_points = get_match_points_from_goal(goal);

    base_points + match_points
}

fn  get_base_points(you: &str) -> i32{
    match you {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => todo!()
    }
}

fn get_move<'a>(op:&'a str, goal: &'a str) -> &'a str {
    match (op, goal){
        ("A", "X") => "Z",
        ("A", "Y") => "X",
        ("A", "Z") => "Y",
        ("B", "X") => "X",
        ("B", "Y") => "Y",
        ("B", "Z") => "Z",
        ("C", "X") => "Y",
        ("C", "Y") => "Z",
        ("C", "Z") => "X",
        (&_, _) => todo!(),
    }
}

fn get_match_points_from_moves(op: &str, you: &str) -> i32 {
    match (op, you){
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        (&_, _) => todo!(),
    }
}

fn get_match_points_from_goal(goal: &str) -> i32 {
    match goal {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        &_ => todo!(),
    }
}

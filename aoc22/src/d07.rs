use std::fs::read_to_string;

struct Node {
    name:String,
    parent:Option<&'static Node>,
    children: Vec<Node>,
    size: u64,
    file:bool
}

pub fn d07_01(input:&str) {
    let file = read_to_string(input).unwrap();
    let lines = file.lines();
    
    let mut root = Node{ name: ("/".to_string()), parent:None, children: Vec::new(), size: (0), file: false};
    let mut current_node = root;

    for line in lines{
        let start = &line[..2];
        match start {
            "$ cd" => todo!(),
            "$ ls" => todo!(),
            "dir" => todo!(),
            _ => { 
                let file = parse_file(line, &current_node);
                current_node.children.push(file);
            },
        }
    }

}

fn parse_file(line:&str, current_node: &'static Node) -> Node {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let (size, filename) = (parts[0], parts[1]);
    Node{name: filename.to_string(), parent: Some(current_node), children: Vec::new(), size: size.parse().unwrap(), file: true}
}
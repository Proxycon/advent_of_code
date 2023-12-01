use std::fs::read_to_string;

#[derive(Debug)]
struct Node {
    name:String,
    parent:Option<&'static Node>,
    children: Vec<Node>,
    size: usize,
}

impl Node {
    fn new(name:&str, parent: Option<&Node>, size: usize) -> Self {
        Node { name: name.to_string(), 
            parent: parent, 
            children: Vec::new(), 
            size: size }
    }

    fn add_child(&mut self, node: Node) {
        self.children.push(node);
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
}

pub fn d07_01(input:&str) {
    let file = read_to_string(input).unwrap();
    let lines = file.lines();
    
    let mut root = Node::new("/", None, 0);
    let mut current_node: &Node = &mut root;

    for line in lines{
        let start = &line[..2];
        match start {
            "$ cd" => todo!(),
            "$ ls" => todo!(),
            "dir" => todo!(),
            _ => { 
                let file = parse_file(line, &current_node);
                current_node.add_child(file);
            },
        }
    }
}

fn parse_file(line:&str, current_node: &'static Node) -> Node {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let (size, filename) = (parts[0], parts[1]);
    Node::new(filename, Some(current_node), size.parse().unwrap())
}
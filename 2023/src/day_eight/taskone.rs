use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


struct Node{
    name: String,
    left: String,
    right: String
}

impl Node{
    fn new(line: &str) -> Node{
        let l:Vec<_> = line.split(" = ").collect();
        let t:Vec<_> = l[1].split(", ").collect();
        Node{
            name: l[0].to_string(), 
            left: t[0].replace("(",""), 
            right:t[1].replace(")","")
        }
    }
}

pub fn task() {
    parse_file();
}


fn parse_file(){
    // Create a path to the desired file
    let path = Path::new("./src/day_eight/input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("\nsteps:{}", get_steps(s)),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn get_steps(s: String) -> u32 {
    let lines: Vec<_> = s.split("\r\n").collect();
    let input = lines[0];
    
    let nodes: Vec<Node> = lines[1..]
        .iter()
        .map(|line| Node::new(line))
        .collect();

    let mut node = get_node(&nodes, "AAA");
    let end_node = get_node(&nodes, "ZZZ");
    
    
    let mut steps = 0;
    while node.name != end_node.name{
        let chars = input.chars();
        for c in chars.into_iter(){
            steps += 1;

            match c {
                'L' => node = get_node(&nodes, &node.left),
                'R' => node = get_node(&nodes, &node.right),
                _ => panic!("wut")
            }
            if node.name == end_node.name { return steps;}
        }
    }


    0
}

fn get_node<'a>(nodes: &'a Vec<Node>, name: &str) -> &'a Node {
    return nodes.iter().find(|&n| n.name == name).unwrap();
}
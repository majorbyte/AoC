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
        Ok(_) => {
            let (input, nodes) = get_nodes(&s);
            print!("\nsteps part 1:{}\nsteps part 2:{}", get_steps(&input, &nodes),get_total_steps(&input, &nodes))
        },
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn get_nodes(s: &str) -> (&str, Vec<Node>) {
    let lines: Vec<_> = s.split("\r\n").collect();
    let input = lines[0];
    
    let nodes = lines[1..]
        .iter()
        .map(|line| Node::new(line))
        .collect();

    return (input, nodes);
}

fn get_steps(input: &str, nodes: &Vec<Node>) -> usize {
    let node = get_node(&nodes, "AAA");
    let end_node = get_node(&nodes, "ZZZ");

    return calculate_steps(input, &nodes, &node, Some(&end_node));
}

fn get_total_steps(input: &str, nodes: &Vec<Node>) -> usize {
    let a_nodes: Vec<&Node> = nodes.iter().filter(|&n| n.name.ends_with("A")).collect::<Vec<&Node>>();
    let steps = a_nodes.iter().map(|&n| calculate_steps(input, &nodes, &n, Option::None)).collect::<Vec<_>>();

    return steps.into_iter().fold(1, |total, step| (total * step) / greates_common_divisor(total, step));
}

fn calculate_steps(input: &str, nodes: &Vec<Node>, start_node: &Node, end_node: Option<&Node>)-> usize{
    let mut node = start_node;
    let mut steps = 0;
    loop {
        let chars = input.chars();
        for c in chars.into_iter(){
            steps += 1;

            match c {
                'L' => node = get_node(&nodes, &node.left),
                'R' => node = get_node(&nodes, &node.right),
                _ => panic!("wut")
            }
            if end_node.is_none() && node.name.ends_with("Z") { return steps} // task 2
            else if !end_node.is_none() && node.name == end_node.unwrap().name { return steps;} // task 1
        }
    }
}

fn greates_common_divisor(mut x: usize, mut y: usize) -> usize {
    while y > 0 {
        (x,y) = (y,x % y);
    }
    x
}

fn get_node<'a>(nodes: &'a Vec<Node>, name: &str) -> &'a Node {
    return nodes.iter().find(|&n| n.name == name).unwrap();
}
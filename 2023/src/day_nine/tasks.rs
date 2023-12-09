use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn task() {
    parse_file();
}

fn parse_file(){
    // Create a path to the desired file
    let path = Path::new("./src/day_nine/input.txt");
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
            let nodes = get_nodes(&s);
            let rev_nodes = nodes.clone().into_iter().map(|node| node.into_iter().rev().collect()).collect();
            print!("\nsteps part 1:{}\nsteps part 2:{}", predict_values(nodes),predict_values(rev_nodes));
        },
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn get_nodes(s: &str) -> Vec<Vec<i32>>{
    let lines: Vec<_> = s.split("\r\n").collect();

    lines.iter().map(|line| line.split(" ").into_iter().map(|x| x.parse::<i32>().unwrap()).collect()).collect()
}

fn predict_values(nodes: Vec<Vec<i32>>) -> i32 {
    nodes.into_iter().map(|line| *line.last().unwrap() + predict(line)).sum()
}

fn predict(mut line: Vec<i32>) -> i32 {

    if line.iter().all(|x| x == &0) {
        return *line.last().unwrap();
    }

    line = get_diff(line);
    *line.last().unwrap() + predict(line)
}

fn get_diff(l: Vec<i32>) -> Vec<i32>{
    l.iter().zip(l[1..].iter()).map(|(cur, next)| next - cur).collect::<Vec<_>>()
}
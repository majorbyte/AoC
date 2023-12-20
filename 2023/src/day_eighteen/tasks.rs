use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use itertools::Itertools;


pub fn task() {
    // Create a path to the desired file
    let path = Path::new("./src/day_eighteen/input.txt");
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
            task_1(&s);
         }
    }
}

struct Node{
    direction: char,
    length:i128,
    color: Vec<char>
}

fn node(data:Vec<&str>) -> Node{

    let direction = data[0].chars().collect::<Vec<_>>()[0];
    let length = data[1].parse::<i128>().unwrap();
    let color = data[2].chars().collect_vec();
    return Node{direction, length, color};
}


fn task_1(input: &str) {
    let path:Vec<Node> = input.split("\r\n").map(|l| node(l.split(" ").collect_vec())).collect();


    let mut x:i128 = 0;
    let mut y:i128 = 0;

    let mut vertices = vec![[0,0]];

    path.iter().for_each(|node| {
        //console.dir(node);
        match node.direction{
            'R' => x += node.length,
            'L' => x -= node.length,
            'U' => y -= node.length,
            'D' => y += node.length,
            _ => ()
        }
        vertices.push([x,y])
    });

    let mut left:i128 = 0;
    let mut right:i128 = 0;
  
    //for (let i = 0; i < vertices.length - 1; i++) {
    for i in 0..vertices.len()-1 {
      left += vertices[i][0] * vertices[i + 1][1];
      right += vertices[i][1] * vertices[i + 1][0];
    }
  
    let mut shoelace = (right - left) / 2;
  
    if shoelace < 0 {shoelace = shoelace * -1;}

    let length = path.iter().fold(0,|p,c| p + c.length);
  
    print!("{}", shoelace + length / 2 + 1);
}
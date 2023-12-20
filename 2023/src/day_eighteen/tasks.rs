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
            task_2(&s);
         }
    }
}

struct Node{
    direction: char,
    length:i128
}

fn node1(data:Vec<&str>) -> Node{
    let direction = data[0].chars().collect::<Vec<_>>()[0];
    let length = data[1].parse::<i128>().unwrap();
    return Node{direction, length};
}

fn node2(data:Vec<&str>) -> Node{
    let color = data[2].chars().collect_vec();
    let direction = match color[7]{
        '0' =>  'R',
        '1' =>  'D',
        '2' =>  'L',
        '3' =>  'U',
        _ => panic!("unknown")
    };
    let src = color[2..7].iter().collect::<String>();
    let length = i128::from_str_radix(&src, 16).unwrap();

    return Node{direction, length};
}


fn task_1(input: &str) {
    let path:Vec<Node> = input.split("\r\n").map(|l| node1(l.split(" ").collect_vec())).collect();
    println!("Task 1 answer: {}",calc(path));
}

fn task_2(input: &str) {
    let path:Vec<Node> = input.split("\r\n").map(|l| node2(l.split(" ").collect_vec())).collect();
    println!("Task 2 answer: {}",calc(path));
}


fn calc(path:Vec<Node>) -> i128{
    let mut x:i128 = 0;
    let mut y:i128 = 0;

    let mut vertices = vec![[0,0]];

    path.iter().for_each(|node| {
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
  
    for i in 0..vertices.len()-1 {
      left += vertices[i][0] * vertices[i + 1][1];
      right += vertices[i][1] * vertices[i + 1][0];
    }
  
    let mut shoelace = (right - left) / 2;
  
    if shoelace < 0 {shoelace = shoelace * -1;}

    let length = path.iter().fold(0,|p,c| p + c.length);
  
    shoelace + length / 2 + 1
}
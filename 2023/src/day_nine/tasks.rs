use std::collections::HashMap;
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

    return lines.iter().map(|line| line.split(" ").into_iter().map(|x| x.parse::<i32>().unwrap()).collect()).collect();
}

fn predict_values(nodes: Vec<Vec<i32>>) -> i32 {
    return nodes.into_iter().map(|line| predict(line)).sum();
}

fn predict(line: Vec<i32>) -> i32 {

    let mut l: Vec<i32> = line.clone();
    let mut map = HashMap::new();
    let mut done: bool = l.clone().iter().all(|x| x == &l[0]);
    let mut steps = 0;

    let mut x = 0;
    println!("");
    x = 0;
    while x < l.len()
    {
        print!(" {}", l[x]);
        x += 1;
    }

    map.insert(steps, l.clone());
    while !done {
        steps += 1;
    
        let n: Vec<i32> = l.clone().into_iter().zip(l[1..].iter()).map(|(cur, next)| next - cur).collect();

        println!("");
        x = 0;
        while x < n.len()
        {
            print!(" {}", n[x]);
            x += 1;
        }
        done = n.iter().all(|x| x == &n[0]);
        l = n.clone();
        map.insert(steps, n);


    } 
    if steps == 0 {return *l.last().unwrap(); }
    let mut next: Vec<i32> = map.get(&(steps)).unwrap().clone();
    

    next.push(next[0]);
    println!("");
    x = 0;
    while x < next.len()
    {
        print!(" {}", next[x]);
        x += 1;
    }
    while steps >= 1{
        let c = map.get(&steps).unwrap().clone();
        
        next = map.get(&(steps-1)).unwrap().clone();

        let v = next.last().unwrap() + c.last().unwrap();

        next.push(v);

        println!("\n {}",v);
        x = 0;
        while x < next.len()
        {
            print!(" {}", next[x]);
            x += 1;
        }

        map.remove(&(steps-1));
        map.insert(steps-1, next.clone());

        steps -= 1;
    }   

    println!("\n{}",next.last().unwrap());
    *next.last().unwrap()
}


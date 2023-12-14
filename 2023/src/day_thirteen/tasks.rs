use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn task() {

    // Create a path to the desired file
    let path = Path::new("./src/day_thirteen/input.txt");
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
            task_1(s.clone());
            task_2(s);
         }
    }
}

fn task_1(input: String){
    let blocks: Vec<&str> = input.split("\r\n\r\n").collect();

    let cost:usize = blocks.iter().map(|b| get_overhead(b)).sum();
    println!("task 1 cost is: {}",cost);
}

fn task_2(input: String){
    let blocks: Vec<&str> = input.split("\r\n\r\n").collect();

    let cost:usize = blocks.iter().map(|b| get_cost_smudges(b)).sum();
    println!("task 2 cost is: {}",cost);
}


fn get_overhead(block: &str )-> usize{

    let cost = get_cost_vertical(&block,None);
    if cost != 0 { return cost; }
    
    get_cost_horizontal(&block,None) * 100
}

fn get_cost_horizontal(block: &str, old_cost:Option<usize>) -> usize {
    let mut map: HashMap<&str, char> = HashMap::new();
    let mut pattern: Vec<char> = vec![];
    block.lines().enumerate().for_each(|(i, k)|{
        if !map.contains_key(k) {
            let x = (i as u8 +65 ) as char;
            map.insert(k, x);
            pattern.push(x)
        } else {
            pattern.push(*map.get(k).unwrap());
        }
    });

    let indexes_to_check: Vec<usize>  = pattern[0..pattern.len()-1].iter().enumerate().filter(|(x, _)|  pattern[*x] == pattern[x+1] ).map(|(x,_)| x).collect();

    for i in indexes_to_check{
        let mut x = i;
        let mut y = i+1;
        let mut f = true;
        while x > 0 && y < pattern.len() && f {
            y +=1;
            x -=1;
            if y < pattern.len() {f = pattern[x] == pattern[y];}
        }

        if f && (y == pattern.len() || x == 0 )  { 
            if old_cost.is_none() {return i+1;}

            if old_cost.unwrap() != (i+1) * 100 {return i+1;}

        } 
    }
    0
}
fn get_cost_vertical(block: &str, old_cost:Option<usize>) -> usize {
    let mut map: HashMap<String, char> = HashMap::new();
    let mut pattern: Vec<char> = vec![];

    let mut patterns: Vec<String> = vec![];

    let lines:Vec<Vec<char>> = block.split("\r\n").map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    for x in 0..lines[0].len(){
        let mut p:Vec<char> = vec![];
        for y  in 0..lines.len(){
            p.push(lines[y][x]);
        }
        patterns.push(p.into_iter().collect());
    }
    patterns.iter().enumerate().for_each(|(i, k)|{
        if !map.contains_key(k) {
            let x = (i as u8 +65 ) as char;
            map.insert(k.to_string(), x);
            pattern.push(x)
        } else {
            pattern.push(*map.get(k).unwrap());
        }
    });

    let indexes_to_check: Vec<usize>  = pattern[0..pattern.len()-1].iter().enumerate().filter(|(x, _)|  pattern[*x] == pattern[x+1] ).map(|(x,_)| x).collect();

    for i in indexes_to_check{
        let mut x = i;
        let mut y = i+1;
        let mut f = true;
        while x > 0 && y < pattern.len() && f {
            y +=1;
            x -=1;
            if y < pattern.len() {f = pattern[x] == pattern[y];}
        }

        if f && (y == pattern.len() || x == 0)  {
            if old_cost.is_none() {return i+1;}

            if old_cost.unwrap() != i+1 {return i+1;}
        } 
    }
    0
}



fn get_cost_smudges(block: &str) -> usize {
    
    let mut lines:Vec<Vec<char>> = block.split("\r\n").map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let old_cost = get_overhead(&block);
    let mut cost = old_cost.clone();
    for y in 0..lines.len(){
        for x in 0..lines[y].len(){
            let c = lines[y][x];
            if c == '.' {lines[y][x] = '#';}
            else {lines[y][x] = '.';}
            let s = &lines.iter().map(|l| l.iter().collect::<String>()).collect::<Vec<String>>()[..];
            let mut co = get_cost_vertical ( &s.join("\r\n"), Some(old_cost));
            if co > 0 && !co.eq(&old_cost) {cost = co;}
            
            co = get_cost_horizontal ( &s.join("\r\n"), Some(old_cost)) *100;
            if co > 0 && !co.eq(&old_cost) {cost = co;}
            
            lines[y][x] = c;
        }
    }

    cost
}



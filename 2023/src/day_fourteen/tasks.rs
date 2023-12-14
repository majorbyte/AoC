use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn task() {

    // Create a path to the desired file
    let path = Path::new("./src/day_fourteen/input.txt");
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


fn task_1(input: &str){
    let mut patterns: Vec<String> = vec![];

    let lines:Vec<Vec<char>> = input.split("\r\n").map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    for x in 0..lines[0].len() {
        let mut y = lines.len();
        let mut p:Vec<char> = vec![];
        while y>0{
            y-=1;
            p.push(lines[y][x]);
        }
        patterns.push(p.into_iter().collect());
    }

    let mut cost = 0;
    for pattern in patterns{
        let chunks = pattern.split("#");
        let mut sorted_chunks:Vec<String> = vec![];
        for chunk in chunks{
            let mut c = chunk.chars().collect::<Vec<char>>();
            c.sort();
            sorted_chunks.push(c.into_iter().collect::<String>());
        }
        let c =sorted_chunks.join("#").chars().collect::<Vec<char>>();
        let mut x = c.len();
        while x > 0{
            x -=1;

            if c[x] != 'O' {continue;}
            
            cost += x+1;
        }

    }
    println!("{:?}",cost);
}

fn task_2(input: &str){
    let mut lines:Vec<String> = input.split("\r\n").map(|l| l.chars().collect::<String>()).collect::<Vec<String>>();

    let mut mem: HashMap<String,Vec<String>> = HashMap::new();
    let mut map: HashMap<String, char> = HashMap::new();

    let mut i:u8 = 65;

    let end = (1_000_000_000 -125)%130 + 125; // manual pattern detection in VS Code... now to figure out how to detect the pattern
    for _ in 0..end{ 
        let k = lines.clone();
        let key = get_key(&k);
        if mem.contains_key(&key) {
            lines = mem.get(&key).unwrap().to_owned(); 

            let mapkey = get_key(&lines);
            if map.contains_key(&mapkey){
                //print!("{}", map.get(&mapkey).unwrap());
            } else{
                let c = (i as u8) as char;
                i+=1;
                map.insert(mapkey,c);
                //print!("{}", c);
            }

            continue;
        }
        lines = spin_cycle(lines.clone());
        mem.insert(key,lines.clone());

        let mapkey = get_key(&lines);
        if map.contains_key(&mapkey){
//            print!("{}", map.get(&mapkey).unwrap());
        } else{
            let c = (i as u8) as char;
            i+=1;
            map.insert(mapkey,c);
        }
    }
    lines = transpose(lines);

    let mut cost = 0;
    for line in lines{

        let c = line.chars().collect::<Vec<char>>();
        let mut x = c.len();
        while x > 0{
            x -=1;
    
            if c[x] != 'O' {continue;}
            
            cost += x+1;

        }
    }
    println!("{}",cost);

}

fn spin_cycle(input: Vec<String>) -> Vec<String>{
    let mut patterns: Vec<String>;

    //north
    patterns = transpose(input);
    patterns = tilt(patterns);

    //west
    patterns = transpose(patterns);
    patterns = tilt(patterns);


    //south
    patterns = transpose(patterns);
    patterns = tilt(patterns);


    //east
    patterns = transpose(patterns);
    patterns = tilt(patterns);

    patterns    
}

fn transpose(lines:Vec<String>)->Vec<String>{
    let mut patterns: Vec<String> = vec![];

    let grid = lines.into_iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    for x in 0..grid[0].len() {
        let mut y = grid.len();
        let mut p:Vec<char> = vec![];
        while y>0{
            y-=1;
            p.push(grid[y][x]);
        }
        patterns.push(p.into_iter().collect());
    }
    patterns
}

fn tilt(patterns:Vec<String>) -> Vec<String> {
    let mut sorted_patterns: Vec<String> = vec![];

    for pattern in patterns{
        let chunks = pattern.split("#");
        let mut sorted_chunks:Vec<String> = vec![];
        for chunk in chunks{
            let mut c = chunk.chars().collect::<Vec<char>>();
            c.sort();
            sorted_chunks.push(c.into_iter().collect::<String>());
        }
        sorted_patterns.push(sorted_chunks.join("#"));
    }

    sorted_patterns

}

fn get_key(pattern: &Vec<String>) -> String{
       format!("{:?}",pattern)
   }
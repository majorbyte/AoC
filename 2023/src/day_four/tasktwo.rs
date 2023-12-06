use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

pub fn task() {
    parse_file();
}

fn parse_file(){
    // Create a path to the desired file
    let path = Path::new("./src/day_four/input.txt");
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
        Ok(_) => print!("\nsum:{}", get_points(s)),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn get_points(s: String) -> i64{

    let v: Vec<&str> = s.split("\r\n").collect();
    let mut sum: i64 = 0;

    let mut numbers: Vec<i32> = vec![];

    for x in v.into_iter(){
        numbers.push(parse_line(x));
    }

    let mut cards: Vec<i32> = vec![];
    cards.push(1);
    for x in 0..=numbers.len()-1{
        if x > cards.len()-1 {
            cards.push(1);
        }
        let size = cards[x];
        for y in 1..=numbers[x] as usize {
            let pos = x + y;
            if pos > cards.len()-1{
                cards.push(size+1);
            } else {
                cards[pos] += size;
            }
        }
    }
    
    for x in cards.into_iter(){
       sum += x as i64; 
    }
    return sum;
}

fn parse_line(s: &str) -> i32{
    let o = s.replace("  ", " 0");
    let v: Vec<&str> = o.split(" | ").collect();
    if v.len() != 2 {return 0;}
    let out_numbers: HashSet<&str> = v[1].split(" ").collect();
    let g: Vec<&str> = v[0].split(": ").collect();
    
    let winning_numbers: HashSet<&str> = g[1].split(" ").collect();

    let intersection = winning_numbers.intersection(&out_numbers);

    let l = intersection.count() as i32;

    return l;    
}


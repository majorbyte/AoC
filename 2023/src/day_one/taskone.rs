use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;


pub fn task() {
    parse_file();
}

fn parse_file(){
    // Create a path to the desired file
    let path = Path::new("./src/day_one/input.txt");
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
        Ok(_) => println!("{} contains:{}", display, get_sum_improved(replace(s))),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn replace(s: String) -> String{
    let re = Regex::new(r"[^.0-9\s]").unwrap();
    return re.replace_all(&s, "").to_string();
}

fn get_sum_improved(input: String) -> u32{
    return  input.split("\r\n").map(|s| get_sum(s)).sum();
}

fn get_sum(s: &str) -> u32{
    if s.len() == 0 {return 0;}

    let first = s.chars().nth(0).unwrap().to_string();
    let last = s.chars().last().unwrap().to_string();
    return (first + &last ).parse::<u32>().unwrap()
}


use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;


pub fn task() {
    parse_file();
}

fn parse_file(){
    // Create a path to the desired file
    let path = Path::new("./src/day_one/input1.txt");
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
        Ok(_) => print!("{} contains:\n{}", display, get_sum(replace(s))),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn replace(s: String) -> String{
    let re = Regex::new(r"[^.0-9\s]").unwrap();
    return re.replace_all(&s, "").to_string();
}

fn get_sum(s: String) -> u128{
    let v: Vec<&str> = s.split("\r\n").collect();
    let mut sum: u128 = 0;
    for x in v.into_iter(){
        let l = x.len();
        match l {
            0 => (),
            1 => {
                let n: String = x.to_owned();
                sum += (n.clone() + &n).parse::<u128>().unwrap();
            },
            2 => sum +=  x.parse::<u128>().unwrap(),
            _ => {
                let first = x.chars().nth(0).unwrap().to_string();
                let last = x.chars().last().unwrap().to_string();
                sum += (first + &last ).parse::<u128>().unwrap()
            }
        }
    }   
    return sum
}


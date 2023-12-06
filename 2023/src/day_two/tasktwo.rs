use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn task() {
    parse_file();
}

fn parse_file(){
    // Create a path to the desired file
    let path = Path::new("./src/day_two/input.txt");
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
        Ok(_) => print!("sum:{}", get_valid_sum(s)),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn get_valid_sum(s: String) -> i64{

    let v: Vec<&str> = s.split("\r\n").collect();
    let mut sum: i64 = 0;

    for x in v.into_iter(){
        sum += parse_line(x);
    }

    return sum;
}

fn parse_line(s: &str) -> i64{
    let v: Vec<&str> = s.split(": ").collect();

    let tries: Vec<&str> = v[1].split("; ").collect();
    
    let mut green = 0;
    let mut blue = 0;
    let mut red = 0;
    
    for x in tries.into_iter(){
        green = get_number(x, "green", green);
        blue = get_number(x, "blue", blue);
        red = get_number(x, "red", red);
    }

    return green * blue * red;
}

fn get_number(s: &str, color: &str, val: i64 ) -> i64{
    /*
    Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games? 
     */

    let v: Vec<&str> = s.split(", ").collect();
    for x in v.into_iter(){
        let t: Vec<&str> = x.split(" ").collect();
        if t[1] == color{
            let n = t[0].parse::<i64>().unwrap();
            if n > val{ return n;}
            return val;
        }
    }
    return val;
}
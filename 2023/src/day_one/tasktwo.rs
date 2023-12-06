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
        Ok(_) => print!("{} contains:\n{}", display, replace(s)),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn replace(s: String) -> u128{
    let v: Vec<&str> = s.split("\r\n").collect();
    let mut sum: u128 = 0;
    let re = Regex::new(r"[^.0-9\s]").unwrap();

    for x in v.into_iter(){
        let n = transform(x);
        sum += get_number(re.replace_all(&n, "").to_string());
    }
    
    return sum;
}

fn transform(s: &str) -> String{
    let numbers = vec!["zero","one","two","three","four","five","six","seven","eight","nine"];
    let mut f = find_smallest_number(s);
    let mut n: String = s.to_owned();

    if f == "" {return s.to_owned();}

    while f != ""{
        let number = numbers.iter().position(|&r| r==f).unwrap().to_string();

        n = n.replacen(&f[..f.len()-1], &number,1);
        
        f = find_smallest_number(&n);

    }

    return n;
}


fn find_smallest_number(s: &str) -> &'static str {
    let numbers = vec!["one","two","three","four","five","six","seven","eight","nine"];

    let mut index: usize = 99999999;
    let mut ret: &str = "";

    for x in numbers.into_iter(){
        let i = s.find(x);

        if i != None && i.unwrap() < index 
        {
            index = i.unwrap();
            ret = x;
        }
    }

    return ret;
}



fn get_number(s: String) -> u128{
    let l = s.len();
    match l {
        0 => return 0,
        1 => {
            let n: String = s.to_owned();
            return (n.clone() + &n).parse::<u128>().unwrap();
        },
        2 => return  s.parse::<u128>().unwrap(),
        _ => {
            let first = s.chars().nth(0).unwrap().to_string();
            let last = s.chars().last().unwrap().to_string();
            return (first + &last ).parse::<u128>().unwrap()
        }
    }

}


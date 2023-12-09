use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;
use std::time::Instant;


pub fn task() -> String{
    parse_file()
}

fn parse_file() -> String {
    let part_time = Instant::now();
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
        Ok(_) => {
            let data = replace(&s);
            return format!("answer part 1:{} (in {:?})\nanswer part 2:{} (in {:?})", get_sum_task1(data.clone()),part_time.elapsed(),get_sum_task2(data) ,part_time.elapsed())
        },
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn replace(s: &str) -> String{
    let re = Regex::new(r"[^.0-9\s]").unwrap();
    re.replace_all(&s, "").to_string()
}

fn get_sum_task1(input: String) -> u32{
    input.split("\r\n").map(|s| get_sum(s)).sum()
}

fn get_sum_task2(input: String) -> u32{
    input.split("\r\n").map(|n| get_sum(&transform(n))).sum()
}


fn get_sum(s: &str) -> u32{
    if s.len() == 0 {return 0;}

    let first = s.chars().nth(0).unwrap().to_string();
    let last = s.chars().last().unwrap().to_string();
    return (first + &last ).parse::<u32>().unwrap()
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

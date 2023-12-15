use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn task() {

    // Create a path to the desired file
    let path = Path::new("./src/day_fiveteen/input.txt");
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
    println!("task 1: {}", input.split(",").map(|s|  get_hash(s) ).sum::<u32>());
}

fn task_2(input: &str){
//    let blocks: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut map: Vec<Vec<&str>> = vec![];
    (0..256).for_each(|_| map.push(vec![]) );

    input.split(",").for_each(|str| {
        let c:Vec<_> = str.chars().collect();
        
        if c[c.len()-1] == '-' {
            let parts = str.split("-").collect::<Vec<&str>>();
            let hash = get_hash(parts[0]) as usize;

            let mut t =  map[hash].clone();
            let i = t.iter().position(|&p| &p.split("=").collect::<Vec<&str>>()[0] == &parts[0] );
            if i.is_some() {
                t.remove(i.unwrap());
                map[hash as  usize] = t;
            }
        }
        if c[c.len()-2] == '=' {
            let parts = str.split("=").collect::<Vec<&str>>();
            let hash = get_hash(parts[0]) as usize;

            let mut t =  map[hash].clone();
            let i = t.iter().position(|&p| &p.split("=").collect::<Vec<&str>>()[0] == &parts[0] );
            if i.is_some() {
                t[i.unwrap()] = &str;
                map[hash] = t;
            } else {
                t.push(&str);
                map[hash] = t;

            }
        }

    });
    let r:usize = map.iter().enumerate().map(|(b, str)|{
        str.iter().enumerate().map(|(i, part)|{
            
            let p = part.split("=").collect::<Vec<&str>>()[1];
            p.parse::<usize>().unwrap() * (i+1) * (b+1)
        }).sum::<usize>()
    }).sum();
    println!("task 2: {}",r);
}

fn get_hash(str: &str) -> u32{
    str.chars().fold(0,|a, c|{
        ((a + c as u32) * 17) %256
    })
}
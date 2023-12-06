use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn task() {
    parse_file();
}

fn parse_file(){
    // Create a path to the desired file
    let path = Path::new("./src/day_three/input.txt");
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
        Ok(_) => print!("\npart:{}", get_parts(s)),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn get_parts(s: String) -> i64{

    let v: Vec<&str> = s.split("\r\n").collect();

    let len = v.len() - 1;
    let mut part_number: String = "".to_string();
    let mut part_sum: i64 = 0;

    for n in 1..len {
        let c0: Vec<char> = v[n-1].chars().collect();
        let c1: Vec<char> = v[n].chars().collect();
        let c2: Vec<char> = v[n+1].chars().collect();

        let clen = c1.len();
        let mut x = 1;
        let mut num: String;
        num = "".to_string();

        let mut touches_part = false;
        while x < clen {

            match c1[x] {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    touches_part = is_part(c0[x-1]) || is_part(c0[x]) || is_part(c0[x+1]) ||
                                  is_part(c1[x-1])       ||             is_part(c1[x+1]) ||
                                  is_part(c2[x-1]) || is_part(c2[x]) || is_part(c2[x+1]);

                },
                '.' => {
                    if (num.len() > 0) && touches_part {
                        part_number +=  " ";
                        part_number += &num;
                        part_sum +=  &num.parse::<i64>().unwrap();
                    }
                    touches_part = false;
                    num = "".to_string();
                },
                _ => {
                    if (num.len() > 0) && touches_part {
                        part_number +=  " ";
                        part_number += &num;
                        part_sum +=  &num.parse::<i64>().unwrap();
                    }
                    touches_part = false;
                    num = "".to_string();
                },
            }
            if touches_part {
                let mut advanced = false;
                while x < clen-1 && is_part_number(c1[x]) {
                    num += &c1[x].to_string();
                    x += 1;
                    advanced = true;
                }
                if advanced {x -=1;}
            }
            else if is_part_number(c1[x]) {
                num += &c1[x].to_string();
            }
            x +=1;
        }

    }

    return part_sum;
}

fn is_part(c: char) -> bool{
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
            return false;
        },
        _ => return true,
    }    
}
fn is_part_number(c: char) -> bool{
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            return true;
        },
        _ => return false,
    }    
}
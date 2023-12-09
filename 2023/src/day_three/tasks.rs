use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

pub fn task() -> String{
    parse_file()
}

fn parse_file()-> String{
    let part_time = Instant::now();
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
        Ok(_) => {
            return format!("\nanswer part 1:{} (in {:?})\nanswer part 2:{} (in {:?})", get_parts_1(s.clone()), part_time.elapsed(),get_parts_2(s) ,part_time.elapsed())
        }
    }
    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn get_parts_1(s: String) -> i64{

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

fn get_parts_2(s: String) -> i128{

    let v: Vec<&str> = s.split("\r\n").collect();

    let len = v.len() - 1;
    let mut part_sum: i128 = 0;

    for n in 1..len {
        let c0: Vec<char> = v[n-1].chars().collect();
        let c1: Vec<char> = v[n].chars().collect();
        let c2: Vec<char> = v[n+1].chars().collect();

        let clen = c1.len();
        let mut x = 1;
        let mut num: String;

        while x < clen {

            if c1[x] != '*'{
                x +=1;
                continue;
            }
            let mut numbers: Vec<i128> = vec![];
            let mut start ;
            let mut end ;
            let mut f = true;
            /*  */
            if is_part_number(c0[x-1]) && is_part_number(c0[x]) && is_part_number(c0[x+1]) {
                num = "".to_string();
                num += &c0[x-1].to_string();
                num += &c0[x].to_string();
                num += &c0[x+1].to_string();
                
                numbers.push(num.parse::<i128>().unwrap());
            } else {
                if is_part_number(c0[x-1]) { 
                    start = x-2;    
                    while is_part_number(c0[start]) {
                        start -=1;
                    }    
                    end = x+2;
                    if end > clen {end = clen;}
        
                    numbers.push(get_part_number(&c0[start..end]));
                    f = false;
                }
                if is_part_number(c0[x+1])|| (is_part_number(c0[x]) && f) {
                    start = x;    
                    end = x+4;
                    if end > clen {end = clen;}
                    numbers.push(get_part_number(&c0[start..end]));
                 }
            }

            if is_part_number(c1[x-1]) {
                start = x-2;    
                while is_part_number(c1[start]) {
                    start -=1;
                }    
                end = x;
                if end > clen {end = clen;}
                numbers.push(get_part_number(&c1[start..end]));

            }
            if is_part_number(c1[x+1]) {
                start = x+1;    
                end = x+4;
                if end > clen {end = clen;}
                numbers.push(get_part_number(&c1[start..end]));

            }
            f = true;
            if is_part_number(c2[x-1]) && is_part_number(c2[x]) && is_part_number(c2[x+1]) {
                num = "".to_string();
                num += &c2[x-1].to_string();
                num += &c2[x].to_string();
                num += &c2[x+1].to_string();
                
                numbers.push(num.parse::<i128>().unwrap());
            } else {
                if is_part_number(c2[x-1]) {
                    start = x-2;
                    while is_part_number(c2[start]) {
                        start -=1;
                    }    
                    end = x+2;
                    if end > clen {end = clen;}

                    numbers.push(get_part_number(&c2[start..end]));
                    f = false;
 
                } 
                if is_part_number(c2[x+1])|| (is_part_number(c2[x]) && f) {
                    start = x;    
                    end = x+4;
                    if end > clen {end = clen;}

                    numbers.push(get_part_number(&c2[start..end]));
                
                }
            }

            if numbers.len() == 2{
                part_sum += numbers[0] * numbers[1];
            }
            x +=1;
        }
    }

    return part_sum;
}


fn get_part_number(line: &[char]) -> i128{
    let mut num: String;
    num = "".to_string();

    for n in line{
        if is_part_number(*n){
            num += &n.to_string();
        } else if num.len() > 0 {
            return num.parse::<i128>().unwrap()
        }
    }
    
    return num.parse::<i128>().unwrap()
}
//part:554003
//part:87263515
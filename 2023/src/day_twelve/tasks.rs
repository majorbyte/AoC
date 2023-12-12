use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

pub fn task() {
    let part_time = Instant::now();

    // Create a path to the desired file
    let path = Path::new("./src/day_twelve/input.txt");
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
            task_1(s,part_time);
         }
    }
}

fn task_1(input:String, time:Instant){
    let count = input.lines().fold(0, |t, line| t + parse_line(line));

    println!("\npart 1 {}  (in {:?})", count, time.elapsed());
}

fn parse_line(line: &str) -> usize{

    let parts = line.split(" ").collect::<Vec<&str>>();
    let mask = parts[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let p = &parts[0].chars().collect::<Vec<char>>()[..];
    let n = &(vec!['?'])[..];
    let p1 = [p,n,p,n,p,n,p,n,p].concat();

    let l = &mask.len();
    let m = mask.into_iter().cycle().take(l*5).collect::<Vec<usize>>();

    let mut mem:Vec<(String,usize)> =vec![];
    let x = count(&p1,&m, &mut mem);
    x
}

fn count(pattern: &[char], mask: &Vec<usize>, mem: &mut Vec<(String, usize)>) -> usize{

    let k1: String = pattern.to_vec().into_iter().collect();
    let key:String = format!("{}:{}", k1, mask.clone().into_iter().map(|x| x.to_string()).collect::<String>());
    let result = mem.iter().find(|k| k.0 == key);
    if !result.is_none() {return result.unwrap().1;}

    let r: usize;
    if pattern.len() == 0 {
        r = if mask.len() == 0 {1} else {0};
        mem.push((key,r));
        return r;
    }
    if mask.len() == 0 {
        r =if pattern.iter().any(|c| c == &'#') {0} else {1};
        mem.push((key,r));
        return r;
    }

    if pattern.len() < mask.iter().sum::<usize>() + mask.len() - 1 {
        r = 0;
        mem.push((key,r));
        return r;
    }

    if pattern[0] == '.' {
        let p = &pattern[1..];
        r = count(p, mask,mem);
        mem.push((key,r));

        return r;
    }
    if pattern[0] == '#' {
        let mut remain =mask.clone();
        let m = remain.remove(0);
        let mut i = 0;
        while i < m{
            if pattern[i] == '.' { return 0; }
            i += 1;
        }
        if m < pattern.len() &&  pattern[m as usize] == '#' { return 0; }

        let x:usize = m+1;

        if x > pattern.len(){
            r = count(&[], &remain,mem);
            mem.push((key,r));
    
            return r;    
        }

        let p = &pattern[x..pattern.len()];
        r = count(p, &remain,mem);
        mem.push((key,r));

        return r;
    }
    let n = vec!['#'];
    let p = &pattern[1..];
    let p1 =[&n[..],p].concat();
    let m = vec!['.'];
    let p2 =[&m[..],p].concat();



    let x = count(&p1, &mask, mem);
    let y =  count(&p2, &mask, mem);
    mem.push((key,x+y));
    x+y
}
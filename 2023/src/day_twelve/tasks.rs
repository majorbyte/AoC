use std::collections::HashMap;
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
            task_1(&s, part_time);
            task_2(s, part_time);
         }
    }
}

fn task_1(input:&String, time:Instant){
    let count = input.lines().fold(0, |t, line| t + parse_line(line,1));

    println!("\npart 1 {}  (in {:?})", count, time.elapsed());
}
fn task_2(input:String, time:Instant){
    let count = input.lines().fold(0, |t, line| t + parse_line(line,5));

    println!("\npart 2 {}  (in {:?})", count, time.elapsed());
}

fn parse_line(line: &str, cnt: usize) -> usize{

    let parts = line.split(" ").collect::<Vec<&str>>();
    let mask = parts[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut q = parts[0].chars().collect::<Vec<char>>();
    let n = &q.clone();
    for _x in 1..cnt{
        q.push('?');
        n.iter().for_each(|c| q.push(*c));
    }
    
    let l = &mask.len();
    let m = mask.into_iter().cycle().take(l*cnt).collect::<Vec<usize>>();

    let mut mem:HashMap<String, usize> = HashMap::new();
    count(&q,&m, &mut mem)
}

fn count(pattern: &[char], mask: &[usize], mem: &mut HashMap<String, usize>) -> usize{

    let k1: String = pattern.to_vec().into_iter().collect();
    let key:String = format!("{}:{}", k1, mask.into_iter().map(|x| x.to_string()).collect::<String>());
    if mem.contains_key(&key) {return *mem.get(&key).unwrap();}

    let r: usize;
    if pattern.len() == 0 {
        r = if mask.len() == 0 {1} else {0};
        if !mem.contains_key(&key) { mem.insert(key,r); }
        return r;
    }
    if mask.len() == 0 {
        r =if pattern.iter().any(|c| c == &'#') {0} else {1};
        if !mem.contains_key(&key) { mem.insert(key,r); }
        return r;
    }

    if pattern.len() < mask.iter().sum::<usize>() + mask.len() - 1 {
        r = 0;
        if !mem.contains_key(&key) { mem.insert(key,r); }
        return r;
    }

    if pattern[0] == '.' {
        let p = &pattern[1..];
        r = count(p, mask,mem);
        if !mem.contains_key(&key) { mem.insert(key,r); }

        return r;
    }
    if pattern[0] == '#' {
        let remain = &mask[1..mask.len()];
        let m = mask[0];
        let mut i = 0;
        while i < m{
            if pattern[i] == '.' { return 0; }
            i += 1;
        }
        if m < pattern.len() &&  pattern[m as usize] == '#' { return 0; }

        let x:usize = m+1;

        if x > pattern.len(){
            r = count(&[], &remain,mem);
            if !mem.contains_key(&key) { mem.insert(key,r); }
    
            return r;    
        }

        let p = &pattern[x..pattern.len()];
        r = count(p, &remain,mem);
        if !mem.contains_key(&key) { mem.insert(key,r); }

        return r;
    }

    let mut v: Vec<char> = vec!['#'];
    pattern[1..].iter().for_each(|c| v.push(*c));

    let x = count(&v, &mask, mem);

    v = vec!['.'];
    pattern[1..].iter().for_each(|c| v.push(*c));

    let y =  count(&v, &mask, mem);

    if !mem.contains_key(&key) { mem.insert(key,x+y); }
    x+y
}
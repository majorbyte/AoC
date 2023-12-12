use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

pub fn task() {

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
            task_1(&s);
            task_2(s);
         }
    }
}

fn task_1(input:&String){
    let part_time = Instant::now();

    let count = input.lines().fold(0, |t, line| t + parse_line(line,1));

    println!("\npart 1 {}  (in {:?})", count, part_time.elapsed());
}
fn task_2(input:String){
    let part_time = Instant::now();

    let count = input.lines().fold(0, |t, line| t + parse_line(line,5));

    println!("\npart 2 {}  (in {:?})", count, part_time.elapsed());
}

fn parse_line(line: &str, cnt: usize) -> usize{
    let parts = line.split(" ").collect::<Vec<&str>>();
    let mask = parts[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut q = parts[0].chars().collect::<Vec<char>>();
    for _x in 1..cnt{
        q.push('?');
        parts[0].chars().for_each(|c| q.push(c));
    }

    let mut known:HashMap<String, usize> = HashMap::new();

    let len = &mask.len();
    let m = mask.into_iter().cycle().take(len*cnt).collect::<Vec<usize>>();

    mem_count(&q,&m, &mut known)
}

fn mem_count(pattern: &[char], mask: &[usize], known: &mut HashMap<String,usize>) -> usize {
    let key = get_key(pattern, mask);
    if known.contains_key(&key) {
        return known.get(&key).unwrap().to_owned(); 
    }

    let cnt = count(pattern, mask, known);
    if !known.contains_key(&key) { known.insert(key,cnt); }

    return cnt;
}

fn count(pattern: &[char], mask: &[usize], known: &mut HashMap<String, usize>) -> usize{

    if pattern.len() == 0 { return if mask.len() == 0 {1} else {0}; }
    if mask.len() == 0 { return if pattern.iter().any(|c| c == &'#') {0} else {1}; }

    if pattern.len() < mask.iter().sum::<usize>() + mask.len() - 1 { return 0; }

    match pattern[0] {
        '.' => {
            return mem_count(&pattern[1..], mask,known);
        },
        '#' => {
            let remain = &mask[1..mask.len()];

            if pattern[0..mask[0]].iter().any(|c| c == &'.') { return 0}

            if mask[0] < pattern.len() &&  pattern[mask[0] as usize] == '#' { return 0; }

            if mask[0]+1 > pattern.len() {  return mem_count(&[], &remain,known); }

            let p = &pattern[mask[0]+1..pattern.len()];
            return mem_count(p, &remain,known);
        },
        _ => ()
    }

    let mut v: Vec<char> = vec!['#'];
    pattern[1..].iter().for_each(|c| v.push(*c));

    let x = mem_count(&v, &mask, known);

    v = vec!['.'];
    pattern[1..].iter().for_each(|c| v.push(*c));

    let y =  mem_count(&v, &mask, known);

    x+y
}

fn get_key(pattern: &[char], mask: &[usize]) -> String{
 //   let k1: String = pattern.to_vec().into_iter().collect();
 //   format!("{}:{}", k1, mask.into_iter().map(|x| x.to_string()).collect::<String>())
    format!("{:?}{:?}",pattern,mask)
}
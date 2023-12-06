use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

pub fn task() {
    parse_file();
}

fn parse_file(){
    // Create a path to the desired file
    let path = Path::new("./src/day_five/input.txt");
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
        Ok(_) => print!("\nlocation:{}", get_location(s)),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn get_location(s: String) -> i64{

    let v: Vec<&str> = s.split("\r\n\r\n").collect();

    let seed_soil: Vec<i64> = map_line(v[1]);
    let soil_fertilizer: Vec<i64> = map_line(v[2]);
    let fertilizer_water: Vec<i64> = map_line(v[3]);
    let water_light: Vec<i64> = map_line(v[4]);
    let light_temperature: Vec<i64> = map_line(v[5]);
    let temperature_humidity: Vec<i64> = map_line(v[6]);
    let humidity_locatio: Vec<i64> = map_line(v[7]);

    let mut seeds: Vec<i64> = v[0].replace("seeds: ", "")
        .split(" ")
        .filter_map(|x| i64::from_str(x).ok())
        .collect();

    seeds = map(seed_soil, seeds);
    seeds = map(soil_fertilizer, seeds);
    seeds = map(fertilizer_water, seeds);
    seeds = map(water_light, seeds);
    seeds = map(light_temperature, seeds);
    seeds = map(temperature_humidity, seeds);
    seeds = map(humidity_locatio, seeds);

    let mut min =2147483647;
    for n in seeds.iter(){
        if n < &min { min = *n;}
    }
    return min;
}

fn map_line(s: &str) -> Vec<i64>{
    let v: Vec<&str> = s.split(":\r\n").collect();
    return v[1].replace("\r\n", " ").split(" ")
        .filter_map(|x| i64::from_str(x).ok())
        .collect();
    }

fn map(t: Vec<i64>, s: Vec<i64>) -> Vec<i64>{
    let mut x = 0;
    let mut ret: Vec<i64> = vec![];
    let mut remnants: Vec<i64> = s.clone();
    while x < t.len(){
        let diff = t[x] - t[x+1];

        let limit = t[x+1] + t[x+2];

        for n in s.iter(){
            if n >= &t[x+1] && n <= &limit{
                let index =remnants.iter().position(|&r| r == *n);
                if index != None{
                    remnants.remove(index.unwrap());
                } else{
                    print!(".not found {}.",n);
                }
                ret.push(n + diff);
            }
        }

        x += 3;
    }
    for n in remnants.iter(){
        ret.push(*n);
    }
    return ret; 
}
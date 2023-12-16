use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use itertools::Itertools;

#[derive(Debug)]
struct Node{
    c: char,
    x: usize,
    y: usize,
    e: bool,
}    

impl Node{
    fn energized(&mut self){
        self.e = true;
    }
    fn de_energize(&mut self){
        self.e = false;
    }
}



pub fn task() {

    // Create a path to the desired file
    let path = Path::new("./src/day_sixteen/input.txt");
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
    let mut grid: Vec<Vec<Node>> = input.lines().enumerate().map(|(y,l)| l.chars().enumerate().map(|(x,c)| Node{c,x,y,e:false} ).collect_vec()).collect_vec();

    
    walk(0,0,0,&mut grid);

    let mut sum = 0;
    for line in &grid{
        for node in line{
            if node.e {sum+=1}
        }
    }
    println!("task 1: {}",sum);   
}

fn task_2(input: &str){
    let mut grid: Vec<Vec<Node>> = input.lines().enumerate().map(|(y,l)| l.chars().enumerate().map(|(x,c)| Node{c,x,y,e:false} ).collect_vec()).collect_vec();

    
    let mut sum = 0;

    for x in 0..grid[0].len(){
        println!("x {}",x);
        let s = walk(x, 0,1,&mut grid);
        if s > sum {sum = s;}
        de_energize(&mut grid);
        print!("a");
        let s = walk(x, grid.len()-1,3,&mut grid);
        if s > sum {sum = s;}
        de_energize(&mut grid);
    }
    println!("x");
    for y in 0..grid.len(){
        let s = walk(0, y,0,&mut grid);
        if s > sum {sum = s;}
        de_energize(&mut grid);
        let s = walk(grid[0].len()-1, y,2,&mut grid);
        if s > sum {sum = s;}
        de_energize(&mut grid);
    }

    println!("{}",sum);
}

fn de_energize(grid: &mut Vec<Vec<Node>>){
    
    for x in 0..grid[0].len(){
        for y in 0..grid.len(){
            grid[y][x].de_energize();
        }
    
    }
    
}

fn walk(s_x:usize,s_y:usize, d:u8, grid: &mut Vec<Vec<Node>>) -> u32 {    
        let mut to_visit: VecDeque<((usize,usize), u8)> = VecDeque::new();
        let mut visited: Vec<(usize,usize, u8)> = vec![];
        to_visit.push_front(((s_x,s_y), d));
    
        let mut item: Option<_> = to_visit.pop_back();
        while item.is_some() || to_visit.len()>0 {
            to_visit.make_contiguous();
    
            let (current_node,direction) = item.unwrap();
            let (mut x, mut y) = (current_node.0, current_node.1);
    
            if visited.contains(&(x,y,direction)) && grid[y][x].e {
                item = to_visit.pop_back();
                continue;
            }
            visited.push((x,y,direction));
    
            match direction{
                1 => {
                    while y < grid.len(){
                        grid[y][x].energized();
                        let node = &grid[y][x];
                        match node.c {
                            '-' => {
                                if node.x > 0 {
                                    to_visit.push_front(((x-1,y),2));
                                }
                                if node.x < grid[0].len()-1{
                                    to_visit.push_front(((x+1,y),0));
                                }
                                break;
                            },
                            '/' => {
                                if node.x >0{
                                    to_visit.push_front(((x-1,y),2));
                                }
                                break;
                            },
                            '\\' => {
                                if node.x < grid[0].len()-1 {
                                    to_visit.push_front(((x+1,y),0));
                                }
                                break;
                            },
            
                            _ => ()
                        }
                        y+=1;
                    }
                },           
                3 => {
                    y+=1;
                    while y >0{
                        y-=1;
                        grid[y][x].energized();
                        let node = &grid[y][x];
                        match node.c {
                            '-' => {
                                if node.x > 0 {
                                    to_visit.push_front(((x-1,y),2));
                                }
                                if node.x < grid[0].len()-1{
                                    to_visit.push_front(((x+1,y),0));
                                }
                                break;
                            },
                            '/' => {
                                if node.x < grid[0].len() -1{
                                    to_visit.push_front(((x+1,y),0));
                                }
                                break;
                            },
                            '\\' => {
                                if node.x >0{
                                    to_visit.push_front(((x-1,y),2));
                                }
                                break;
                            },
            
                            _ => ()
                        }
                    }
    
                },
                2 => {
                    x+=1;
                    while x >0{
                        x-=1;
                        grid[y][x].energized();
                        let node = &grid[y][x];
                        match node.c {
                            '|' => {
                                if node.y > 0 {
                                    to_visit.push_front(((x,y-1),3));
                                }
                                if node.y < grid.len()-1{
                                    to_visit.push_front(((x,y+1),1));
                                }
                                break;
                            },
                            '/' => {
                                if node.y < grid.len()-1{
                                    to_visit.push_front(((x,y+1),1));
                                }
                                break;
                            },
                            '\\' => {
                                if node.y > 0 {
                                    to_visit.push_front(((x,y-1),3));
                                }
                                break;
                            },
            
                            _ => ()
                        }
                    }
                },
                0 => {
                    while x < grid[0].len(){
                        grid[y][x].energized();
                        let node = &grid[y][x];
                        match node.c {
                            '|' => {
                                if node.y > 0 {
                                    to_visit.push_front(((x,y-1), 3));
                                }
                                if node.y < grid.len()-1{
                                    to_visit.push_front(((x,y+1),1));
                                }
                                break;
                            },
                            '/' => {
                                if node.y > 0 {
                                    to_visit.push_front(((x,y-1),3));
                                }
                                break;
                            },
                            '\\' => {
                                if node.y < grid.len()-1{
                                    to_visit.push_front(((x,y+1),1));
                                }
                                break;
                            },
            
                            _ => ()
                        }
                        x+=1;
                    }
            
                },
                _ => ()
            }
            item = to_visit.pop_back();
        }
    
        let mut sum = 0;
        for line in grid{
            for node in line{
                if node.e {sum+=1}
            }
        }
    sum
}
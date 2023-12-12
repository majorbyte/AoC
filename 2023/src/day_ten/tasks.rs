use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::VecDeque;
use std::usize;

enum Directions{
    Left,
    Right,
    Up,
    Down
}

pub fn task() {
    // Create a path to the desired file
    let path = Path::new("./src/day_ten/input.txt");
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
            task_2(task_1(s.clone()));
         }
    }
}

fn task_1(input: String) ->  Vec<Vec<char>>{
    let mut grid = input
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect::<Vec<Vec<char>>>();


    if let Some((s_row, s_col)) = get_start_point(&grid) {
         
        let mut numsteps = 0;
        let mut to_explore: VecDeque<(usize, usize)> = VecDeque::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        
        // We can see our pipe as a two legged tree breadth first search will let us walk the tunnel, 
        // the number of iterations is how far away we are, and once we can't go anywhere, we're done
        // https://en.wikipedia.org/wiki/Breadth-first_search
        queue.push_back((s_row, s_col));
        while queue.len() > 0 {
            to_explore.append(&mut queue);
            while to_explore.len() > 0 {
                if let Some((i, j)) = to_explore.pop_front() {
                    queue.append(&mut explore(&mut grid, i, j));
                }
            }
            if queue.len() > 0 {
                numsteps += 1;
            }
        }
        println!("\nFurthest away: {}", numsteps);
    }
    return grid;
}

fn task_2(grid: Vec<Vec<char>>){

    let mut tile_count = 0;

    for y in 0..grid.len(){
        let mut intersections = 0;
        let mut x = 0;

        while x < grid.len() && match grid[y][x] {
            '│' | '┌' | '└' => false,
            _ => true
        }{
            x+=1;
        }

        let mut i = grid.len()-1;
        while i > x && match grid[y][i] {
            '│' | '┐' | '┘' => false,
            _ => true
        }{
            i-=1;
        }
        

        while x<i{
            let cell = grid[y][x];
            match cell { 
                'S' | '│' => intersections += 1,
                '┌' => {
                    (x, intersections) = handle_cell(&grid, x, y, intersections, '┐','┘');
                },
                '└' => {
                    (x, intersections) = handle_cell(&grid, x, y, intersections, '┘','┐');
                },
                _ => ()
            }
            let is_viable =  match grid[y][x] {
                '|' | '-' | '7' | 'J' | '.' | 'F' | 'L' => true,
                _ => false,
            };
            if intersections % 2 == 1 && is_viable {
                tile_count += 1;
            }
            x+=1;
        }
    }

    print!("number of enclosed tiles: {}",tile_count);
}

fn handle_cell(grid: &Vec<Vec<char>>, x: usize, y: usize, intersections: i32, c1: char, c2: char) -> (usize, i32) {
    let mut x = x;
    let mut intersections = intersections;

    x += 1;
    let mut cell = grid[y][x];
    while cell == '─'  {
        x += 1;
        cell = grid[y][x];
    }
    if cell == c1 { intersections += 2; }
    if cell == c2 { intersections += 1; }

    return (x, intersections);
}


fn get_start_point(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                return Some((y, x));
            }
        }
    }

    None
}

fn get_valid_directions(c: char) -> Vec<Directions> {
    match c {
        'S' => vec![Directions::Right,Directions::Down, Directions::Left, Directions::Up],
        '─' => vec![Directions::Right, Directions::Left],
        '│' => vec![Directions::Down, Directions::Up],
        '┌' => vec![Directions::Right, Directions::Down],
        '└' => vec![Directions::Right, Directions::Up],
        '┐' => vec![Directions::Left, Directions::Down],
        '┘' => vec![Directions::Left, Directions::Up],
        _ => vec![],
    }
}

fn mark_visited(grid: &mut Vec<Vec<char>>, i:&usize, j:&usize)  {
    grid[*i][*j] = match grid[*i][*j] {
        '-' => '─',
        '|' => '│',
        'F' => '┌',
        'L' => '└',
        '7' => '┐',
        'J' => '┘',
        _ => grid[*i][*j],
    }
}

fn get_new_coords(y: usize, x: usize, d:&Directions) -> (usize,usize){
    match d {
        Directions::Right => (y, x+1),// (0,1)
        Directions::Left => (y, x-1),// (0,-1)
        Directions::Up => (y-1,x),// (1,0)
        Directions::Down => (y+1,x),// (-1,0)
    }
}

fn get_new_directions(grid: &mut Vec<Vec<char>>, y: usize, x: usize, d: &Directions) -> Option<(usize, usize)> {
    let new_coords = get_new_coords(y, x, d);
    if new_coords.0 > grid.len() && new_coords.1 > grid[0].len() { return None;}

    let target = &grid[new_coords.0][new_coords.1];

    match d {
        Directions::Right => match target {
            '-' | '7' | 'J' => Some(new_coords),
            _ => None,
        },
        Directions::Down => match target {
            '|' | 'J' | 'L' => Some(new_coords),
            _ => None,
        },
        Directions::Left => match target {
            '-' | 'L' | 'F' => Some(new_coords),
            _ => None,
        },
        Directions::Up => match target {
            '|' | '7' | 'F' => Some(new_coords),
            _ => None,
        }
    }
}

fn explore(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> VecDeque<(usize, usize)> {
    let directions = get_valid_directions(grid[i][j]);

    let valid_neighbours: VecDeque<(usize, usize)> = directions
        .iter()
        .filter_map(|d| get_new_directions(grid, i, j, &d))
        .collect();

    for (i, j) in &valid_neighbours {
        mark_visited(grid, i, j);
    }

    return valid_neighbours;
}
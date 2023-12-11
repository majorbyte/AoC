use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec;

struct Node{
    x: usize,
    y: usize,
    val:char,
    cost: u32,
}

impl Node{
    fn set_cost(&mut self, cost:u32){
        self.cost = cost;
    }

    fn equals(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y 
    }
}

pub fn task() {
    // Create a path to the desired file
    let path = Path::new("./src/day_eleven/input.txt");
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
            task_1(s);
         }
    }
}

fn task_1(input: String){
    let grid = get_grid(input,2);
    
    let mut rows: Vec<usize> = vec![];
    let mut cols: Vec<usize> = vec![];

    cols = grid.iter().filter(|r| r.y == 0 && r.cost > 1 ).map(|r| r.x).collect();
    rows = grid.iter().filter(|r| r.x == 0 && r.cost > 1 ).map(|r| r.y).collect();

    let galaxies = grid.iter().filter(|n| n.val == '#').collect::<Vec<&Node>>();

    let mut pairs: Vec<(&Node,&Node)> = vec![];
    /*for p1 in galaxies.iter(){
        println!("{}",pairs.len());
        for p2 in galaxies.iter(){
            if p1.equals(p2) {continue;}

            let found = pairs.iter().find(|pair| (pair.0.equals(p1) && pair.1.equals(p2)) || (pair.1.equals(p1) && pair.0.equals(p2)));
            if !found.is_none() {continue;}

            pairs.push((p1,p2));
        }
    }*/

    for (i, el1) in galaxies.iter().enumerate() {
        for el2 in galaxies[i+1..].iter() {
            if el1.equals(el2) {continue;}
            pairs.push((el1,el2));
        }
    }

    let mut total_cost_1 = 0;
    let mut total_cost_2 = 0;
    for pair in pairs.iter(){

        /*
            cost = abs(x1-x2) + abs(y1-y2) +  number of expensive lines
        
         */
        let cost = pair.0.x.abs_diff(pair.1.x) + pair.0.y.abs_diff(pair.1.y);

        let rows_crossed = rows.iter().filter(|r| (&pair.0.y > r  && &pair.1.y < r ) || (&pair.1.y > r  && &pair.0.y < r ) ).count();
        let cols_crossed = cols.iter().filter(|r| (&pair.0.x > r  && &pair.1.x < r ) || (&pair.1.x > r  && &pair.0.x < r ) ).count();

        total_cost_1 += cost + rows_crossed + cols_crossed;
        total_cost_2 += cost + rows_crossed * (1000000-1)  + cols_crossed * (1000000-1);

    }

    print!("\ncost 1: {}\ncost 2: {}", total_cost_1, total_cost_2);

}

fn get_grid(input: String, cost:u32) -> Vec<Node> {

    let mut grid = input
    .lines()
    .enumerate()
    .map(|(y, line)| line
        .chars()
        .enumerate()
        .map(|(x, c)| Node{x,y,val:c,cost:1})
        .collect::<Vec<Node>>())
    .collect::<Vec<Vec<Node>>>();

    fix_column_cost(&mut grid, cost);
    fix_row_cost(&mut grid,cost);
    
    grid.into_iter().flatten().collect()
}

fn fix_column_cost(grid: &mut Vec<Vec<Node>>, cost:u32) {
    let mut y = grid.len()-1;
    while y > 0{
        let mut x = 0;
        while x < grid[y].len(){
            if grid[y][x].val == '#' { 
                x+=1;
                break; 
            }
            x+=1;
        }
        if x == grid[y].len() && grid[y][x-1].val != '#' {
            x = 0;
            while x < grid[y].len(){
                grid[y][x].set_cost(cost);
                x+=1;
            }                
        }
        y -= 1;
    }
}


fn fix_row_cost(grid: &mut Vec<Vec<Node>>, cost:u32) {
    let mut x = grid[0].len()-1;
    while x > 0{
        let mut y = 0;
        while y < grid.len(){
            if grid[y][x].val == '#' { 
                y+=1;
                break; 
            }
            y+=1;
        }
        if y == grid.len() && grid[y-1][x].val != '#' {
            y = 0;
            while y < grid.len(){
                grid[y][x].set_cost(cost);
                y+=1;
            }                
        }
        x -= 1;
    }}

/*
fn fix_row_cost(grid: &mut Vec<Vec<&mut Node>>, cost:u32) {
    grid
        .into_iter()
        .filter(|row| row.iter().all(|c| c.val != '#'))
        .map(|nodes| nodes.into_iter().map(|node | node.set_cost(cost)));
}
*/
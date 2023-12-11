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

    fn distance(&self, other: &Self) -> usize{
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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
            tasks(s);
         }
    }
}

fn tasks(input: String){
    let grid = get_grid(input);
    
    let rows: Vec<usize>= grid.iter().filter(|r| r.x == 0 && r.cost > 1 ).map(|r| r.y).collect();
    let cols: Vec<usize> = grid.iter().filter(|r| r.y == 0 && r.cost > 1 ).map(|r| r.x).collect();

    let galaxies = grid.iter().filter(|n| n.val == '#').collect::<Vec<&Node>>();

    let mut pairs : Vec<(&Node,&Node)> = vec![]; 

    galaxies.iter().enumerate().for_each(|(i, left)|  
        galaxies[i+1..].iter().filter(|right| !left.equals(right)).for_each(|right| 
            pairs.push((left,right))));

    let t1 = pairs.iter().fold(0, |t, (left,right) | t + cost(&rows,&cols,&left, &right,1) );
    let t2 = pairs.iter().fold(0, |t, (left,right) | t + cost(&rows,&cols,&left, &right,999_999) );

    print!("\ncost 1: {}\ncost 2: {}", t1, t2);
}

fn cost(rows: &Vec<usize>, cols: &Vec<usize>, left:&Node, right:&Node, tax: usize) -> usize{
    left.distance(right) + (row_count(rows, &left.y, &right.y) +  col_count(cols, &left.x, &right.x)) * tax
}
fn row_count(rows: &Vec<usize>, ly:&usize, ry:&usize) -> usize{
    rows.iter().filter(|r| (ly > r  && ry < r ) || (ry > r  && ly < r ) ).count()
}
fn col_count(cols: &Vec<usize>, lx:&usize, rx:&usize) -> usize{
    cols.iter().filter(|r| (lx > r  && rx < r ) || (rx > r  && lx < r ) ).count()
}


fn get_grid(input: String) -> Vec<Node> {

    let mut grid = input.lines()
    .enumerate()
    .map(|(y, line)| {
        let cost = if line.chars().all(|c| c == '.') { 2 } else { 1 };
        line
            .chars()
            .enumerate()
            .map(|(x, c)| Node{x,y,val:c,cost})
            .collect::<Vec<Node>>()
    })
    .flatten()
    .collect::<Vec<Node>>();

    fix_costs(&mut grid);

    grid
}

fn fix_costs(nodes: &mut  Vec<Node>){

    let size = (nodes.len() as f64).sqrt() as usize;
    
    for x in 0..size{
        if nodes.iter().filter(|node| node.x == x).any(|node| node.val != '.') {continue;}

        nodes.iter_mut().filter(|node| node.x==x).for_each(|node| node.set_cost(2));
    }
}
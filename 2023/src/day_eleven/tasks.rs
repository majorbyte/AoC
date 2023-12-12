use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;


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
    let part_time = Instant::now();

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
            tasks(s,part_time);
         }
    }
}

fn tasks(input: String, time: Instant){
    let grid = get_grid(input);
    
    let rows: Vec<usize>= grid.iter().filter(|r| r.x == 0 && r.cost > 1 ).map(|r| r.y).collect();
    let cols: Vec<usize> = grid.iter().filter(|r| r.y == 0 && r.cost > 1 ).map(|r| r.x).collect();

    let galaxies = grid.iter().filter(|n| n.val == '#').collect::<Vec<&Node>>();

    let (t1,t2) = galaxies.iter().enumerate().flat_map(|(i, &left)|  
            galaxies[i+1..].iter().filter(|&right| !&left.equals(&right)).map(|&right| (left,right)).collect::<Vec<_>>())
        .fold((0,0), |(t1, t2), (left,right) | cost(&rows,&cols,&left, &right,1,999_999, t1, t2) );

    print!("\ncost 1: {}\ncost 2: {} in {:?}", t1, t2,time.elapsed());
}

fn cost(rows: &Vec<usize>, cols: &Vec<usize>, left:&Node, right:&Node, cost_1: usize, cost_2: usize, total_1: usize, total_2: usize) -> (usize, usize){
    let d =  left.distance(right);
    let cnt = row_count(rows, &left.y, &right.y) +  col_count(cols, &left.x, &right.x);
    
    (total_1 + d + cnt*cost_1, total_2 + d + cnt*cost_2)
}
fn row_count(rows: &Vec<usize>, ly:&usize, ry:&usize) -> usize{
    rows.iter().filter(|&r| (ly > r  && ry < r ) || (ry > r  && ly < r ) ).count()
}
fn col_count(cols: &Vec<usize>, lx:&usize, rx:&usize) -> usize{
    cols.iter().filter(|&r| (lx > r  && rx < r ) || (rx > r  && lx < r ) ).count()
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
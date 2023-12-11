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
    
    let rows: Vec<usize>= grid.iter().filter(|r| r.x == 0 && r.cost > 1 ).map(|r| r.y).collect();
    let cols: Vec<usize> = grid.iter().filter(|r| r.y == 0 && r.cost > 1 ).map(|r| r.x).collect();

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

    let lines = input.lines();

    let mut grid = lines
    .enumerate()
    .map(|(y, line)| line
        .chars()
        .enumerate()
        .map(|(x, c)| Node{x,y,val:c,cost:1})
        .collect::<Vec<Node>>())
    .flatten()
    .collect::<Vec<Node>>();

    fix_costs(&mut grid, cost);

    grid
}

fn fix_costs(nodes: &mut  Vec<Node>, cost:u32){

    let size = (nodes.len() as f64).sqrt() as usize;
    for n in nodes.chunks_mut(size).filter(|x| x.iter().all(|n| n.val == '.')){
        for node in n.iter_mut()   {
            node.set_cost(cost);
        }
    }

    for x in 0..size{
        let mut chunks = nodes.iter_mut().filter(|node| node.x==x).collect::<Vec<_>>();
        if chunks.iter().any(|x| x.val != '.') {continue;}

        for node in chunks.iter_mut()   {
            node.set_cost(cost);
        }
    }
}
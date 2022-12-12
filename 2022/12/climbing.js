const manhattan = (pos0, pos1) => Math.abs(pos1.x - pos0.x) +  Math.abs(pos1.y - pos0.y);

const neighbours = (grid, node)  => {
  const ret = [];
  const x = node.x;
  const y = node.y;

  if (node.val === 'E') return [];
  if (node.val === 'S') node.cost = 97;

  if (grid[x - 1] && grid[x - 1][y]) {
    const v = grid[x - 1][y].cost - node.cost;
    if (v < 2) ret.push(grid[x - 1][y]);
  }

  if (grid[x + 1] && grid[x + 1][y]) {
    const v = grid[x + 1][y].cost - node.cost;
    if (v < 2) ret.push(grid[x + 1][y]);
  }

  if (grid[x] && grid[x][y - 1]) {
    const v = grid[x][y - 1].cost - node.cost;
    if (v < 2) ret.push(grid[x][y - 1]);
  }
  if (grid[x] && grid[x][y + 1]) {
    const v = grid[x][y + 1].cost - node.cost;
    if (v < 2) ret.push(grid[x][y + 1]);
  }
  return ret;
}


const astar = (grid, start, end) => {
  let openSet = [start];

  const explored = [];

  while (openSet.length > 0) {
    openSet = openSet.sort((a,b) => b.fScore - a.fScore);
    
    const currentNode = openSet.pop();
    if(currentNode == end) {
      return currentNode;
    }

    explored.push(currentNode);

    const neighbors = neighbours(grid, currentNode);
    neighbors.forEach(neighbor => {
      if(explored.indexOf(neighbor) >= 0) return;
      
      const gScore = currentNode.gScore + 1
      
      if(openSet.indexOf(neighbor) == -1) {
        updateNeighbor = true;
        neighbor.hScore = neighbor.hScore || manhattan(neighbor.pos, end.pos);
        neighbor.gScore = gScore;
        neighbor.fScore = neighbor.gScore + neighbor.hScore;
        openSet.push(neighbor);
      }
      else if(gScore < neighbor.gScore) {
        neighbor.gScore = gScore;
        neighbor.fScore = neighbor.gScore + neighbor.hScore;
      }
    });
  }
  console.log("malade");
}

class Node {

  constructor(x,y, val) { 
    this.x = x;
    this.y = y; 
    this.val = val;
    this.cost = val.charCodeAt(0);
    this.pos = {x,y};
    this.fScore = 0; 
    this.gScore = 0; 
    this.hScore = 0;
  }
} 


let start = null;
let goal = null;

const b = [];

const createGraph = data => {
  const graph = [];

  for (let x = 0, xl = data[0].length; x < xl; x++) {
    graph[x] = [];
    for (let y = 0, yl = data.length; y < yl; y++){
      graph[x][y] = new Node(x,y, data[y][x]);
      if (data[y][x] === 'S') start = graph[x][y];
      if (data[y][x] === 'E') goal = graph[x][y];
      if (data[y][x] === 'b') b.push(graph[x][y]); // b will always be linked to an a-node
    }
  }
  return graph;
}
const resetGraph = () => {
  xl = graph.length;
  yl = graph[0].length;

  for (let x = 0; x < xl; x++) 
  for (let y = 0; y < yl; y++){
    graph[x][y].fScore = 0;
    graph[x][y].gScore = 0;
    graph[x][y].hScore = 0;
  }
  return graph;
}
const graph = createGraph(require('fs').readFileSync("climbing.input", { encoding: "utf-8" }).split('\r\n').map(x => x.split('')));
goal.cost = 123;

console.log(astar(graph,start,goal).fScore);

const part2 = [];
for (let x = 0; x < b.length; x++){
  resetGraph();
  part2.push(astar(graph,b[x],goal).fScore);
}

console.log(1 + part2.reduce((p,c) => p > c ? c:p,999 )); // +1  because we start at 'a' and not 'b'

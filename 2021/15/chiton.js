const manhattan = (pos0, pos1) => Math.abs(pos1.x - pos0.x) +  Math.abs(pos1.y - pos0.y);

const neighbours = (grid, node)  => {
  var ret = [];
  var x = node.x;
  var y = node.y;

  if (grid[x - 1] && grid[x - 1][y]) {
    ret.push(grid[x - 1][y]);
  }

  if (grid[x + 1] && grid[x + 1][y]) {
    ret.push(grid[x + 1][y]);
  }

  if (grid[x] && grid[x][y - 1]) {
    ret.push(grid[x][y - 1]);
  }
  if (grid[x] && grid[x][y + 1]) {
    ret.push(grid[x][y + 1]);
  }
  return ret;
}

// https://en.wikipedia.org/wiki/A*_search_algorithm#Algorithm_description

const astar = (grid, start, end) => {
  let openSet = [start];

  const explored = [];

  while (openSet.length > 0) {
    openSet = openSet.sort((a,b) => b.fScore - a.fScore);
    
    const currentNode = openSet.pop();
    if(currentNode == end) {
      console.dir(currentNode);
      return;
    }

    explored.push(currentNode);

    const neighbors = neighbours(grid, currentNode);
    neighbors.forEach(neighbor => {
      if(explored.indexOf(neighbor) >= 0) return;
      
      const gScore = currentNode.gScore + neighbor.cost;
      
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

const node = (x,y, cost) => ({ x, y, cost, pos:{x,y}, fScore: 0, gScore: 0, hScore: 0});

const createGraph = data => {
  const graph = [];

  for (let x = 0, xl = data[0].length; x < xl; x++) {
    graph[x] = [];
    for (let y = 0, yl = data.length; y < yl; y++)
      graph[x][y] = node(x,y, data[y][x]);
  }

  return graph;
}

const createGraph25 = (data,size) => {
  const graph = [];

  for (let n = 0; n < 5; n++) 
  for (let x = 0; x < size; x++) {
    let nx = x+n*size;
    graph[nx] = [];
    
    for (let m = 0; m < 5; m++) 
    for (let y = 0; y < size; y++){
      let ny = y+m*size;
      let cost = data[x][y] + n + m;
      if (cost > 9) cost -= 9; 
      
      graph[nx][ny] = node(nx,ny, cost);
    }
  }

  return graph;
}

const graph = createGraph25(require('fs').readFileSync("chiton.input", { encoding: "utf-8" }).split('\r\n').map(x => x.split('').map(Number)),100);
const start = graph[0][0];
const goal = graph[499][499];
astar(graph,start,goal,false);



const fs = require('fs');
const map = data => {
  let cave = data.split('\r\n').map(x => x.split('').map(Number));
  // set a border around the cave of value 9, so we don't have to perform edge-cases ;)
  cave = cave.map(x => {x.unshift(9);x.push(9); return x});
  let border = [...Array(cave[0].length)].fill(9);
  cave.push(border);
  cave.unshift(border);

  const points = [];
  for(let y = 1; y < cave.length-1;y++)
  for(let x = 1; x < cave[0].length-1;x++){
    let point = cave[y][x];
    if (point < cave[y-1][x] && point < cave[y+1][x] && point < cave[y][x-1] && point < cave[y][x+1] ) points.push([x,y]);
  }

  const floodfill = (x,y,sum) => {
    if (cave[y][x] == 9) return sum;
    cave[y][x] = 9;
    sum++;
    sum = floodfill(x+1,y,sum);
    sum = floodfill(x-1,y,sum);
    sum = floodfill(x,y+1,sum);
    sum = floodfill(x,y-1,sum);
    return sum
  }

  let risk = 0;
  let volumes =[];
  for(let point of points){
    risk += 1 + cave[point[1]][point[0]];
    volumes.push(floodfill(point[0],point[1],0));
  }
  console.log(risk);
  console.log(volumes.sort((a,b) => a < b ? 1 : -1).slice(0,3).reduce((p,c)=> p*c,1));
}

fs.readFile('./map.input', 'utf-8', (_,data) => map(data));
const fs = require('fs');

let grid;
const shouldFlash = _ => grid.find(y => y.find(x => x > 9));

const flash = _ => {
  let count = 0;
  let l = grid[0].length-1;
  for(let y = 1; y < grid.length-1;y++)
  for(let x = 1; x < l;x++){
    if (grid[y][x] < 10) continue;

    grid[y][x] = 0;

    [-1,0,1].forEach(i => [-1,0,1].forEach(j => grid[y+i][x+j] !==0 ? grid[y+i][x+j]++ : false));
    count++;
  }
  return count;
}

const gridSum = _ => {
  let l = grid[0].length-1;
  let sum = 0;
  for(let y = 1; y < grid.length-1;y++)
  for(let x = 1; x < l;x++)
    sum += grid[y][x];

  return sum;
}

const step = _ => {
  let sum = 0;
  let l = grid[0].length-1;
  for(let y = 1; y < grid.length-1;y++)
  for(let x = 1; x < l;x++){
    grid[y][x]++
  }
  while (shouldFlash()){
    sum += flash();
  }
  return sum;
}

const process = (data) => {
  grid = data.split('\r\n').map(x => x = x.split('').map(Number));
  grid = grid.map(x => {x.push(-999);x.unshift(-999);return x;});
  const border = [...Array(grid[0].length)].fill(-999);
  grid.push(border);
  grid.unshift(border);
  
  let cnt = 0;
  let sum = 0;
  while (cnt < 100){
    sum += step();
    cnt++
  }
  console.log(sum);
  while(gridSum() !== 0) {
    step();
    cnt++
  }
  console.log(cnt);
}

fs.readFile('./flash.input', 'utf-8', (_,data) => {
  process(data);
});

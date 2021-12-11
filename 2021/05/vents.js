const fs = require('fs');

const createGrid = (data,diagonal) => {
  data = data
    .replace(/ -> /g,',')
    .split('\r\n')
    .map(x => x.split(',').map(Number));

  const grid = data
    .reduce((p, c) => {
      const dx = Math.sign(c[2] - c[0]);
      const dy = Math.sign(c[3] - c[1]);

      if (!diagonal && dx !== 0 && dy !== 0) return p;

      const stop = dx !== 0 ? Math.abs(c[0]-c[2]) :  Math.abs(c[1]-c[3]);

      for (let n = 0;n <= stop;n++){
        const x = c[0]+dx*n;
        const y = c[1]+dy*n;
        if (!p[x]) p[x] = {};
        p[x][y] = p[x][y] == null ? 1 : p[x][y]+1;  
      }
      return p;
    },{});
  const count = Object.keys(grid).reduce((p1,x) => {
    return p1 + Object.keys(grid[x]).reduce((p2,y) => {
      return p2 + (grid[x][y] > 1 ? 1 : 0);
    },0)
  },0);
  console.dir(count);
}

fs.readFile('./vents.input', 'utf-8', (_,data) => {
  createGrid(data,false);
  createGrid(data,true);
});

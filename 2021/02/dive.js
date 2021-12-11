const fs = require('fs');

const dive = data => data.reduce((p,c)=> {
  let x = Number(c[1]);
  switch(c[0]){
    case 'forward':
      return [p[0]+x,p[1],p[2] + x * p[1] ];
    case 'down':
      return [p[0],p[1]+x,p[2]];
    case 'up':
      return [p[0],p[1]-x,p[2]];
  }
},[0,0,0])

fs.readFile('./dive.input', 'utf-8', (_,data) => {
  const result = dive(data.split('\r\n').map(x => x.split(' ')));
  console.log(result[0]*result[1]);
  console.log(result[0]*result[2]);
});

const fs = require('fs');

const pair = ['(','[','{','<'];
const char = [')',']','}','>'];
const value = [3,57,1197,25137];

const count = (line, corrupted) => {
  let l = [];

  for(const c of line){
    if (pair.indexOf(c)>-1) l.push(c); 
    else if (l[l.length-1] === pair[ char.indexOf(c) ] ) l.splice(-1,1);
    else return corrupted ? value[char.indexOf(c)] : 0;
  }
  return corrupted ? 0 : l.reverse().map(c => char[pair.indexOf(c)]) .reduce((p,c) => 5*p + char.indexOf(c)+1,0 );
}

fs.readFile('./syntax.input', 'utf-8', (_,data) => {
  console.log(data.split('\r\n').reduce((p,c)  => p += count(c.split(''),true)  ,0));
  let score = data.split('\r\n').map(x => count(x,false)).filter(x => x > 0).sort((a,b) => a-b)
  console.log(score [Math.floor(score.length/2)]);
});

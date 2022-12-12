const fs = require('fs');

let x = 1;
let cycle = 0;
const cycles = [20, 60, 100, 140, 180, 220];
let total = 0;

const CRT = [];
let line = '';
let position = 0;

const increaseCycle = () => {
  cycle++;
  if (cycles.includes(cycle)) total += cycle*x;

  if(line.length >= x -1 && line.length <= x +1) line += '#';
  else line += " ";

  if (line.length >= 40)
  {
    console.log(line);
    CRT.push(line);
    line = '';
  }
}

const instructions = (lines) => {

  for(const line of lines){
    if (line === 'noop') increaseCycle();
    else {
      increaseCycle();
      increaseCycle();
      const v = Number(line.split(' ')[1]);
      x += v;
      position = x -1;
    }
  }
  return total;
}

const sprite = function(lines){

}

fs.readFile('./cathode.input', 'utf-8', (_,data) => {
  console.log(instructions(data.split('\r\n')));
});


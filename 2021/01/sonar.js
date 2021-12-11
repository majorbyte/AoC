const fs = require('fs');

const depth = data => {
  let cnt = 0;
  let cnt2 = 0;
  for(let x=1;x<data.length;x++) 
    cnt += data[x] > data[x-1] ? 1 : 0
  for(let x=2;x<data.length-1;x++) 
    cnt2 += (data[x-1] + data[x] + data[x+1]) > (data[x-2] + data[x-1] + data[x]) ? 1 : 0
  
  return [cnt,cnt2];
};

fs.readFile('./sonar.input', 'utf-8', (_,data) => {
  console.log(depth(data.split('\r\n').map(Number)));
});

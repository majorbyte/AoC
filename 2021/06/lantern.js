const fs = require('fs');

const spawn = (input, duration) => {
  const timers = input.split(',').map(Number);
  let school = [0,0,0,0,0,0,0,0,0];
  timers.forEach(x => school[x]++);

  for(let x = duration; x>0; x--){ 
      const size = school.shift();
      school[6] += size;
      school[8] = size;
  }

 return school.reduce((p, c)=> p+c, 0);
}


fs.readFile('./lantern.input', 'utf-8', (_,data) => {
  console.log(spawn(data,80));
  console.log(spawn(data,255));
});
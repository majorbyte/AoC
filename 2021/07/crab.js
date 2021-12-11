const fs = require('fs');

function median(values){
  values.sort(function(a,b){
    return a-b;
  });

  var half = Math.floor(values.length / 2);
  
  if (values.length % 2)
    return values[half];
  
  return (values[half - 1] + values[half]) / 2.0;
}

const distance1 = (data) => {
  data = data.split(',').map(Number);  
  let crabs =[];
  data.forEach(x => {
    if (crabs[x]) crabs[x]++; 
    else crabs[x] = 1;
  });

  let result = median(data);

  let cost = 0;
  for(let x = crabs.length-1; x >=0; x--){
    if (!crabs[x]) continue;
    cost += Math.abs(x-result) * crabs[x];
  }

  return cost;
}

const distance2 = (data) => {
  data = data.split(',').map(Number);  

  let max = Math.max(...data);
  let min = Math.min(...data);
  let cost = 99999999999999;
  for(let c = min; c < max;c++){
    let fuel = 0;
    for(let pos of data){
      const distance = Math.abs(pos - c);
      fuel += (distance*distance+distance)/2;
    }
    if ( fuel < cost) cost = fuel;
  }
  return cost;
}

fs.readFile('./crabs.input', 'utf-8', (_,data) => {
  console.log(distance1(data));
  console.log(distance2(data));
});
const fs = require('fs');

const getDigit = (x,most) => most ? x >= 0 ? 1 : 0 : x < 0 ? 1 : 0; 

const part1 = (data) => {
  const getValue = (data, most) => [...Array(12).keys()].reduce((num,x) => num += getDigit(data.reduce((p,c) =>  p + (c[x] == 1 ? 1 : -1),0),most), '');

  const gamma = getValue(data,true);
  const epsilon = getValue(data,false);
  return parseInt(gamma,2) * parseInt(epsilon,2);
}

const part2 = (data) => {
  const getValue = (data, most) => 
  {
    for(let x = 0; x < 12; x++){
      let num = getDigit(data.reduce((p,c) =>  p + (c[x] == 1 ? 1 : -1),0),most);
      
      data = data.filter(n => n[x] == num );
      if(data.length === 1) return data[0];
    }
  }

  const oxygen = getValue(data,true);
  const co2 = getValue(data, false);
  return parseInt(oxygen,2) * parseInt(co2,2);
}

fs.readFile('./binary.input', 'utf-8', (_,data) =>{
  data = data.split('\r\n');
  console.log(part1(data));
  console.log(part2(data));
});



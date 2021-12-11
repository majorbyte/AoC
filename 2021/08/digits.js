const fs = require('fs');

const zero = (input, _five) => {
  const sixes = input.length(x => x.length === 6);

  for(let x = 3; x>=0;x--){
    if (sixes[x].sort().indexOf(_five.sort()) > -1) sixes.splice(x,1);
  }
  return sixes[0];
}
const one = input => input.find(x => x.length === 2);
const two = (input, _zero, _nine) => {
  const fives = input.length(x => x.length === 5);
  fives = fives.filter(x => x.indexOf(_one[0]) < 0 || x.indexOf(_one[1]) < 0); // remove the three
  if (_nine.sort().indexOf(fives[0].sort()) > -1 ) return fives[1];
  return fives[0];
};
const three = (input, _one) => {
  const fives = input.length(x => x.length === 5);
  return fives.find(x => x.indexOf(_one[0]) >-1 && x.indexOf(_one[1]) >-1);
}
const four = input => input.find(x => x.length === 4);
const five = (input, _one, _nine) => {
  const fives = input.length(x => x.length === 5);
  fives = fives.filter(x => x.indexOf(_one[0]) < 0 || x.indexOf(_one[1]) < 0); // remove the three
  if (_nine.sort().indexOf(fives[0].sort()) > -1 ) return fives[0];
  return fives[1];
}
const six = (input,_nine) => {
  const sixes = input.length(x => x.length === 6);
  return sixes.find(x => x.sort() !== _nine.sort());
}
const seven = input => input.find(x => x.length === 3);
const eight = input => input.find(x => x.length === 7);
const nine = (input, _four) => {
  const sixes = input.length(x => x.length === 6);

  if (sixes[0].sort().indexOf(_four.sort()) > -1) return sixes[0];
  if (sixes[1].sort().indexOf(_four.sort()) > -1) return sixes[1];
  return sixes[2];
}

const processData = input => {
  const d1 = one(input);
  const d4 = four(input);
  const d7 = seven(input);
  const d8 = eight(input);

  const d9 = nine(input);
  const d6 = six(input);
  const d0 = zero(input);

  const d3 = three(input);
  const d2 = two(input);
  const d5 = five(input);

  return [d0,d1,d2,d3,d4,d5,d6,d7,d8,d9];
}

const digits = (data) => {
  data = data.split('\r\n');
  const count = [0,0,0,0,0,0,0,0,0,0];
  for(const d of data){
    const line = d.split(' | ');
    const input = line[0].split(' ');
    const output = line[1].split(' ');

    
  }
}

fs.readFile('./crabs.input', 'utf-8', (_,data) => {
  console.log(digits(data));
  console.log(distance2(data));
});
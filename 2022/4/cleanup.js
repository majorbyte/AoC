const fs = require('fs');

const getPairs = (pairs) => pairs[0].split('-').map(Number).concat(pairs[1].split('-').map(Number));

function findFullPairs(line) {
  const [x1,x2, y1,y2] = getPairs(line.split(','));
  if (x1 <= y1 && x2 >= y2 ) return line;
  if (y1 <= x1 && y2 >= x2 ) return line;
  return null;
}

function findOverlappingPairs(line) {
  const [x1,x2, y1,y2] = getPairs(line.split(','));
  if (x2 >= y1 && x1 <= y2) return line;
  return null;
}

fs.readFile('./cleanup.input', 'utf-8', (_,data) => {
  let result = data.split('\r\n').map(findFullPairs).filter(x => x);
  console.log(result.length);
  result = data.split('\r\n').map(findOverlappingPairs).filter(x => x);
  console.log(result.length);
});
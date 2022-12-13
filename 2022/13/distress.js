function checkPairs (pair){
  const [left,right] = pair;
  for(let x = 0; x<left.length;x++){
    if (right[x] === undefined) return false;

    if (Array.isArray(left[x]) && Array.isArray(right[x])){
      const check = checkPairs([left[x],right[x]]);
      if (check !== null) return check;
    }
    else if (Array.isArray(left[x])){
      const check = checkPairs([left[x],[right[x]]]);
      if (check !== null) return check;
    }
    else if (Array.isArray(right[x])) {
      const check = checkPairs([[left[x]],right[x]]);
      if (check !== null) return check;
    }
    
    if (left[x] - right[x] === 0)  continue;
    return left[x] < right[x];
  }
  return left.length < right.length ? true : null;
}

const data = require('fs').readFileSync("distress.input", { encoding: "utf-8" }).split('\r\n\r\n').map(x => x.split('\r\n').map(x => JSON.parse(x)));

// part1
console.dir(data.reduce((p,c,i) => p + (checkPairs(c) ? i+1 :0), 0));

//part2
const [d1,d2] = [[[2]],[[6]]];
let part2 = data.reduce((p,c) => [...p, c[0], c[1]],[d1,d2]).sort((a,b) => checkPairs([a,b]) ? -1 : 1 );
console.dir((1+part2.indexOf(d1)) * (1+part2.indexOf(d2)));

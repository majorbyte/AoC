let double = false;
const map = data => data.reduce( (map,[from, to]) => (
  map.set(from, (map.get(from) || []).concat(to)), 
  map.set(to, (map.get(to) || []).concat(from))
  )
,new Map());

const isUpperCase = c => c === c.toUpperCase();

const getAvailableSteps = (links, path) => {
  let check = (path,step) => double 
    ? isUpperCase(step) || !path.includes(step) || !path.filter(x => !isUpperCase(x)).some((value, index, arr) => arr.indexOf(value) < index ) 
    : isUpperCase(step) || !path.includes(step)
    ;

  return links.filter(step => step !== 'start' && check(path,step)) ;
}
const parse = (current, path) => {
  if (current == 'end') return 1;    

  let steps = getAvailableSteps(cave.get(current),path);

  return steps.map(step => parse(step, [...path,step])).reduce((p,c) => p+c,0)
}

const cave = map(require('fs').readFileSync("pathing.input", { encoding: "utf-8" }).split('\r\n').map(x => x.split('-')));

console.log(parse('start',[]));
double = true;
console.log(parse('start',[]));
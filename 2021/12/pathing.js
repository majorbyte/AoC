const fs = require('fs');
const path = require('path');

const mapStep = (data, from, to) => data[from] 
  ? data[from].links.push(to)
  : data[from] = {links:[to]}

const map = data => data.reduce((p,c) => {
  const step = c.split('-');
  mapStep(p,step[0],step[1]);
  mapStep(p,step[1],step[0]);
  return p;
},{});


const isUpperCase = c => c === c.toUpperCase();

const getAvailableSteps = (links, path, double) => {

  if (!double) return links.filter(x => isUpperCase(x) || path.indexOf(x) === -1) ;
  return links.filter(x => 
    isUpperCase(x) 
    || (x !== double 
      ? path.indexOf(x) === -1 
      : (path.filter(n => n == double).length < 2 && ['start','end'].indexOf(x) <0)));
}
  

const getStep = (cave, path, paths, double) => {
  const segments = path.split(',');
  const current = segments[segments.length-1];
  if (current == 'end') {
    if (paths.indexOf(path) < 0) paths.push(path);
    return;
  }

  const available = getAvailableSteps(cave[current].links,segments, double);
  for(const pos of available){
    getStep(cave,`${path},${pos}`,paths, double);
  }
  
  return path;
}

const route = (data, allowDouble) => {
  const cave = map(data);
  const paths = []
  if (allowDouble){
    for(const segment of Object.keys(cave).filter(x => !isUpperCase(x) && ['start','end'].indexOf(x) <0 ))
    {  
      console.log(segment)
      for(const pos of cave.start.links){
        getStep(cave,`start,${pos}`,paths,segment);
      } 
    }

  } else {
    for(const pos of cave.start.links){
      getStep(cave,`start,${pos}`,paths);
    }
  }
  console.dir(paths.sort());
  console.log(paths.length);
}


fs.readFile('./pathing.input', 'utf-8', (_,data) => {
  route(data.split('\r\n'),false);
  route(data.split('\r\n'),true);
});

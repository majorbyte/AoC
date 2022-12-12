const fs = require('fs');

const tails = [];
const knotTails =[];
const head = {x:0, y:0};
const tail = {x:0, y:0};

const knots = [{x:0, y:0},{x:0, y:0},{x:0, y:0},{x:0, y:0},{x:0, y:0},{x:0, y:0},{x:0, y:0},{x:0, y:0},{x:0, y:0},{x:0, y:0}];

const parseDirection= (o, d) => {
  switch(d){
    case 'D':
      o.y += -1;
      break;
    case 'U':
      o.y += 1;
      break;
    case 'L':
      o.x += -1;
      break;
    case 'R':
      o.x += 1;
      break;
  }
}

const checkTail = (direction) => {
  if (head.x == tail.x && head.y == tail.y ) return;
  else if (Math.abs(head.x - tail.x) > 1){
    tail.y = head.y;
    parseDirection(tail,direction);
  }
  else if (Math.abs(head.y - tail.y) > 1){
    tail.x = head.x;
    parseDirection(tail,direction);
  }
  if (!tails.some(p => p.x === tail.x && p.y === tail.y)) tails.push({x:tail.x, y:tail.y});
}

const checkPositions = (o1, o2) => {
  if (o1.x == o2.x && o1.y == o2.y ) return;
  else if (Math.abs(o1.x - o2.x) > 1){
    if (Math.abs(o1.y - o2.y) > 1) o2.y += (o1.y - o2.y) > 0 ? 1 : -1; 
    else o2.y = o1.y;
    o2.x += (o1.x - o2.x) > 0 ? 1 : -1;
  }
  else if (Math.abs(o1.y - o2.y) > 1){
    if (Math.abs(o1.x - o2.x) > 1) o2.x += (o1.x - o2.x) > 0 ? 1 : -1;
    else o2.x = o1.x;
    o2.y += (o1.y - o2.y) > 0 ? 1 : -1; 
  }
}

const parseHead = (direction) => {
  parseDirection(head,direction);
  checkTail(direction);
}

const parseKnots = (direction) => {
  parseDirection(knots[0],direction);
  for(let x = 0; x < knots.length-1;x++)
    checkPositions(knots[x], knots[x+1]);
  console.dir(knots);
  const t = knots[9];
  if (!knotTails.some(p => p.x === t.x && p.y === t.y)) knotTails.push({x:t.x, y:t.y});
}

const parseInstruction = (i) => {
  for(let x = 0; x < i[1];x++) 
    //parseHead(i[0]);
    parseKnots(i[0]);
}

const instructions = (lines) => {

  for(const line of lines){
    console.log(line);
    parseInstruction(line.split(' '));
    
  }
  console.log("tails");
  console.dir(knotTails.sort((a,b) => a.x-b.x));
  return knotTails.length

}

fs.readFile('./tail.input', 'utf-8', (_,data) => {
  console.log(instructions(data.split('\r\n')));
});


//target area: x=240..292, y=-90..-57

const minX = 240;
const maxX = 292;
const minY = -90;
const maxY = -57

const hit = (dx,dy) => {
  let x =  y =  maxHeight = 0
  while(x <= maxX && y >= minY){
    x += dx;
    y += dy--;
    if (y > maxHeight) maxHeight = y;
    if (x >= minX && x <= maxX && y >= minY && y <= maxY) return [x,y, maxHeight];
    if (dx > 0) dx--;
    if (dx < 0) dx++;
  }
  return false;
}


let heights = [];
for(let y =minY;y < -minY;y++)
for(let x =0;x <= maxX;x++){
  let result = hit(x,y);
  if (result) heights.push(result);
}

console.log(heights.reduce((p,c) => c[2] > p ? c[2] : p,0));
console.log(heights.length);

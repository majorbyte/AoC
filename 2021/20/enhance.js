let map;
let image;
let flipping = false;
let background;

const number = (y, x ) => image[y] !== undefined ? image[y][x] !== undefined ? image[y][x] : background : background;

const newPoint = (y, x) => {
  let binary = [
    number(y-1,x-1),number( y-1,x),number( y-1,x+1),
    number(y,x-1),number( y,x),number( y,x+1),
    number(y+1,x-1),number(y+1,x),number( y+1,x+1)
  ].join('');
  return map[parseInt(binary,2)];;
}

const enhance = _ => {
  image = [
    Array(image[0].length+2).fill(background),
    ... image.map(line => [background,...line,background]),
    Array(image[0].length+2).fill(background)
  ]

  return image.map((line,y) => line.map((_, x) => newPoint(y,x)));
}
//img.reduce((p,c,i) => [...p, c.reduce((lp,lc,li) => [...lp, newPoint(i, li)] ,[])],[]);

const part1 = data => {

  map = data[0].replace(/\./g,'0').replace(/#/g, '1').split('').map(Number);

  /*
    inifinte image, background filled with 0's
        [0, 0, 0
    n =  0, 0, 0   => pos 1 in map, if that is a #/1, it turns all darkness into light
         0, 0, 0]
    
    same for all 1's => pos x-1 in map, if that's a ./o it would turn all light into dark

    so background starts blinking on each enhance
  */
  
  flipping = map[0] === 1 && map[map.length-1] === 0;
  background= '0';


  image = data[1].split('\r\n').map(x => x.replace(/\./g,'0').replace(/#/g, '1').split(''));
  for (let x = 0; x< 50;x++){
    image = enhance();
    if (flipping) background = background === '0' ? '1' : '0';
  }
  console.dir(image.map(y => y.join('')).join('').replace(/0/g,'').length);
}


result = part1(require('fs')
  .readFileSync("enhance.input", { encoding: "utf-8" })
  .split('\r\n\r\n')  
);
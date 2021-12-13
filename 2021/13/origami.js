
const hasPoint = (arr,point) => JSON.stringify(arr).includes(JSON.stringify(point));


const fold = data => {
  const instructions = data[1].split('\r\n').map(x => x.replace('fold along ','').split('='));
  let paper =  data[0].split('\r\n').map(x => x.split(',').map(Number));
  let log = true;
  paper = instructions.reduce((p,c) => {
    const foldLine = Number(c[1]);
    if (c[0] == 'x'){
      const points = p.filter(x => x[0] > foldLine);
      p = p.filter(x => x[0] < foldLine);
      for(const point of points){
        const newPoint = [foldLine-(point[0]-foldLine), point[1]];
        if (!hasPoint(p,newPoint)) p.push(newPoint);
      }
    } else {
      const points = p.filter(y => y[1] > foldLine);
      p = p.filter(y => y[1] < foldLine);
      for(const point of points){
        const newPoint = [point[0],foldLine-(point[1]-foldLine)];
        if (!hasPoint(p,newPoint)) p.push(newPoint);
      }
    }
    if (log) {
      console.log(p.length);
      log = false
    }
    return p;

  },paper)

  const maxX = paper.reduce((p,c) => c[0] > p ? c[0] : p ,0);
  const maxY = paper.reduce((p,c) => c[1] > p ? c[1] : p ,0);

  for(y = 0; y <= maxY; y++){
    let line = ''
    for(x = 0; x <= maxX; x++){
       line += hasPoint(paper, [x,y]) ? '█' : ' ';
    }
    console.log(line)
  }

}


fold(require('fs').readFileSync("origami.input", { encoding: "utf-8" }).split('\r\n\r\n'));
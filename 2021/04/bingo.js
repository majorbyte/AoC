const fs = require('fs');

const transpose = m => m[0].map((_,i) => m.map(x => x[i]));

const checkNumber = (arr, num) => {
  for (let x = 0; x < 5; x++){
    if(arr[x].indexOf(num) > -1) arr[x].splice(arr[x].indexOf(num),1);
    if (arr[x].length <= 0) return true;
  }
  return false;
}

const bingo = (_,data) => {
  const numbers = data.split('\r\n').slice(0,1)[0].split(',').map(Number);
  const cards = data.split('\r\n\r\n').splice(1).map(x=> {
    const rows =  x.split('\r\n').map(n => n.trim().replace(/  /g,' ').split(' ').map(Number));
    const columns = transpose(rows);
    return {rows,columns}
  });

  for (const x of numbers){
    for(const card of cards){
      let hasBingo = checkNumber(card.rows,x) || checkNumber(card.columns,x);
      if (hasBingo && cards.length == 1) {
        console.log(card.rows.reduce((p,c) => p+c.reduce((x,y) => x+y,0 ) ,0 ) * x);
        return;
      } else if (hasBingo){
        delete card.rows;
      }
    }
    for (let x = cards.length-1; x >=0;x--){
      if (!cards[x].rows) cards.splice(x,1);
    }
  }
}

fs.readFile('./bingo.input', 'utf-8', bingo);

const fs = require('fs');

const rps = ['', 'X','Y','Z'];

const outcome = (a) => {
  switch(a){
    case 'A X': return 3;
    case 'A Y': return 4;
    case 'A Z': return 8; 
    case 'B X': return 1;
    case 'B Y': return 5;
    case 'B Z': return 9;
    case 'C X': return 2;
    case 'C Y': return 6;
    case 'C Z': return 7;
  }
}

const parseResult = (line) => outcome(line);

fs.readFile('./rock.input', 'utf-8', (_,data) => {
  console.log(data.split('\r\n').map(parseResult).reduce((p,c) => p+c,0));
});


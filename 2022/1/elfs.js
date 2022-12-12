const fs = require('fs');

fs.readFile('./elf.input', 'utf-8', (_,data) => {
  console.log(data.split('\r\n\r\n').map(elf => elf.split('\r\n').reduce((p,c) => p+Number(c) ,0)).sort((a,b) =>  b-a) );
});


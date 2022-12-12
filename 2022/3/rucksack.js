const fs = require('fs');



class Rucksack{
  constructor(items){
    this.compartment1 = {};
    this.compartment2 = {};
    for(let x = 0; x < (items.length/2);x++)
      this.compartment1[items[x]] = this.compartment1[items[x]] != null ? this.compartment1[items[x]] +1 : 1;
      
    for(let x = items.length/2; x < items.length;x++)
      this.compartment2[items[x]] = this.compartment2[items[x]] != null ? this.compartment2[items[x]] +1 : 1;
  }

  all(){
    const r = {};
    Object.assign(r ,this.compartment1);
    Object.assign(r ,this.compartment2);
    return r;
  }
}

const getValue = (c) => c.charCodeAt(0) - (c.charCodeAt(0) < 91 ? 38 : 96);

fs.readFile('./rucksack.input', 'utf-8', (_,data) => {

  const sacks = data
  .split('\r\n')
  .map(x => new Rucksack(x));

  console.dir(sacks
    .map(rucksack => Object.keys(rucksack.compartment1)
      .reduce((p,c)=> rucksack.compartment2[c] > 0 ? p + getValue(c)  : p,0) 
    )
    .reduce((p,c) => p+c,0)
  );

  let total = 0;
  for(let x = 0;x < sacks.length;x += 3){
    const a = Object.keys(sacks[x].all())
      .reduce((p,c)=> sacks[x+1].all()[c] > 0 ? p.concat([c]) : p, [])
      .reduce((p,c)=> sacks[x+2].all()[c] > 0 ? p.concat([c]) : p, []);
    total += getValue(a[0]);
  }
  console.log(total)

});


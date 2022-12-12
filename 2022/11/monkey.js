const mod = 2*7*13*3*19*5*11*17;

class Monkey {
  constructor(items, operation, test, where){
    this.items = items;
    this.operation = operation;
    this.test = test;
    this.where = where;
    this.inspections = 0;
  }

  check(i){
    this.inspections++;
    this.operation(i);
    //this.items[i] = Math.floor(this.items[i]/3); //part 1
    this.items[i] = this.items[i] % mod; //part 2
    if (this.items[i] % this.test === 0) return true;
    return false;
  }
}

const monkeys = [
  new Monkey([50, 70, 89, 75, 66, 66],function(i){this.items[i]*=5;},2,[1,2]),
  new Monkey([85],function(i){this.items[i]*=this.items[i];},7,[6,3]),
  new Monkey([66, 51, 71, 76, 58, 55, 58, 60],function(i){this.items[i]+=1;},13,[3,1]),
  new Monkey([79, 52, 55, 51], function(i){this.items[i]+=6;},3,[4,6]),
  new Monkey([69, 92],function(i){this.items[i]*=17;},19,[5,7]),
  new Monkey([71, 76, 73, 98, 67, 79, 99], function(i){this.items[i]+=8},5,[2,0]),
  new Monkey([82, 76, 69, 69, 57], function(i){this.items[i]+=7;},11,[4,7] ),
  new Monkey([65, 79, 86], function(i){this.items[i]+=5;},17,[0,5])
];


const inspectMonkey = (monkey) => {
  for(let x = monkey.items.length-1;x > -1; x--) 
    monkeys[monkey.where[monkey.check(x)+0]].items.push(monkey.items.splice(x,1)[0]);
  }
const inspect = () => monkeys.forEach(inspectMonkey);
for(let x = 0; x<10000;x++) inspect();

console.dir(monkeys) ;

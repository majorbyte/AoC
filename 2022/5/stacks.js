const fs = require('fs');
/*
                [M]     [W] [M]    
            [L] [Q] [S] [C] [R]    
            [Q] [F] [F] [T] [N] [S]
    [N]     [V] [V] [H] [L] [J] [D]
    [D] [D] [W] [P] [G] [R] [D] [F]
[T] [T] [M] [G] [G] [Q] [N] [W] [L]
[Z] [H] [F] [J] [D] [Z] [S] [H] [Q]
[B] [V] [B] [T] [W] [V] [Z] [Z] [M]
 1   2   3   4   5   6   7   8   9 
*/
const stacks = [
  [],
  ['B','Z','T'],
  ['V','H','T','D','N'],
  ['B','F','M','D'],
  ['T','J','G','W','V','Q','L'],
  ['W','D','G','P','V','F','Q','M'],
  ['V','Z','Q','G','H','F','S'],
  ['Z','S','N','R','L','T','C','W'],
  ['Z','H','W','D','J','N','R','M'],
  ['M','Q','L','F','D','S'],
];

/*
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
*/
const example = [
  [],
  ['Z','N'],
  ['M','C','D'],
  ['P']
];


const crateMover9001 = (from, amount) => workingStack[from].splice(workingStack[from].length - amount,amount);
const crateMover9000 = (from, amount) => crateMover9001(from,amount).reverse();

const moveStack = (from, to, amount, crane) => workingStack[to].concat(crane(from, amount));

const processInstruction = (line, crane) => {
  const x = line.split(' ');
  const [amount,from,to] = [x[1],x[3],x[5]].map(Number);
  workingStack[to] = moveStack(from,to,amount,crane);
}

const workingStack = stacks;

fs.readFile('./stacks.input', 'utf-8', (_,data) => {
  data
    .split('\r\n')
    .forEach(line => processInstruction(line,crateMover9001));
    //.forEach(line => processInstruction(line,crateMover9000));
  
    console.log(workingStack.map(x => x[x.length-1]).join(''));
});
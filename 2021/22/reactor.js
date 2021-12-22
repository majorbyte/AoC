
const reactor = {'on':[],'off':[]}

const parseCube = input => input.split('=')[1].split('..').map(Number);

const step = input => {
  let [n, y, z] = input.split(',');
  //on x=10..12
  //y=10..12
  //z=10..12
  let [state, x] = n.split(' ');
  
  x = parseCube(x);
  y = parseCube(y);
  z = parseCube(z);

  console.dir(x);
  console.dir(y);
  console.dir(z);  

  for(let i = x[0], il = x[1]; i <= il; i++)
  for(let j = y[0], jl = y[1]; j <= jl; j++)
  for(let k = z[0], kl = z[1]; k <= kl; k++){
    const val = [i,j,k].toString();
    let other = state == 'on' ? 'off' : 'on';
    let index = reactor[other].indexOf(val);
    if (index > -1) reactor[other].splice(index,1);
    if (reactor[state].indexOf(val) === -1) reactor[state].push(val);
  }

  console.dir(reactor.on.length)
}

const parse = data => data.forEach(step)

parse(require('fs').readFileSync("reactor.input", { encoding: "utf-8" }).split('\r\n'));

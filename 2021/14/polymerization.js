const insert = (data, steps) => {
  let template = data[0];
  const pairs = data[1].split('\r\n').map(x => x.split(' -> ')).map(x => [x[0], [x[0][0] + x[1], x[1]+x[0][1] ]]);
  
  let result = {};
  for(let x = template.length-2; x >= 0 ; x--){
    let current = template[x] + template[x+1];
    if (!result[current]) result[current] = 0
    result[current]++
  }

  let step = 0;
  while (step < steps){
    result = Object.entries(result).reduce((p,[key,cnt]) => {
      const pair = pairs.find(x=>x[0] == key);
      if (!pair) return p;
      const [left, right] = pair[1];
      if (!p[left]) p[left] = 0;
      if (!p[right]) p[right] = 0;
      p[left] += cnt;
      p[right] += cnt;

      return p;
    },{});
    step++;
  }
  let count = Object.entries(result).reduce((p, [key,cnt]) => {
    let letters = key.split('');
    if (!p[letters[0]]) p[letters[0]] =0;
    if (!p[letters[1]]) p[letters[1]] =0;
    p[letters[0]] += cnt; 
    p[letters[1]] += cnt;

    return p;
  },{});

  const mm = Object.entries(count).reduce((p,[key,cnt]) => {
    if(cnt > p[1]) p[1] = cnt;
    if(cnt < p[0]) p[0] = cnt;
    return p;
  },[Infinity,0]);
  console.log(Math.ceil((mm[1]-mm[0])/2));
}

insert(require('fs').readFileSync("polymers.input", { encoding: "utf-8" }).split('\r\n\r\n'),10);
insert(require('fs').readFileSync("polymers.input", { encoding: "utf-8" }).split('\r\n\r\n'),40);

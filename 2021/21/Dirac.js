
let p1 = [7,0];
let p2 = [5,0];

let dice = 1;
while (p1[1]<1000){

  let steps = (dice++ + dice++ + dice++) % 10;
  let score = (steps + p1[0]) % 10 || 10;
  p1[1] += score;
  p1[0] = score;
  console.dir(p1)
  if (p1[1] > 999 ) break;
  steps = (dice++ + dice++ + dice++) % 10;
  score = (steps + p2[0]) % 10 || 10;
  p2[1] += score;
  p2[0] = score;
}
console.dir(--dice * p2[1]);
let sumVersion =0;
const literal = (bits) => {
  let group = bits.splice(0,5);
  let check = Number(group.splice(0,1)[0]);
  let number = [];
  while (check === 1){
    number = [...number,...group];
    group = bits.splice(0,5)
    check = Number(group.splice(0,1)[0]);
  }
  number = [...number,...group];
  return [bits, parseInt(number.join(''),2)];
}

const operator = bits => {
  let length =  Number(bits.splice(0,1)[0]);
  let numbers = [];

  if (length === 0){
    //If the length type ID is 0, then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
    let subpacketLength = parseInt(bits.splice(0, 15).join(''),2);
    let packetBits = bits.splice(0,subpacketLength);
    while (packetBits.length > 0 ){
      let result = decode(packetBits);
      numbers.push(result[1]);
      packetBits = result[0];
    }
  } else {
    //If the length type ID is 1, then the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
    let numberOfPackets = parseInt(bits.splice(0, 11).join(''),2);
    for (; numberOfPackets>0; numberOfPackets--){
      let result = decode(bits);
      bits = result[0]
      numbers.push(result[1]);
    }
  }
  return [bits, numbers];
}

const decode = (bits) => {
  let version = parseInt(bits.splice(0,3).join(''),2);
  sumVersion += version;
  let type = parseInt(bits.splice(0,3).join(''),2);
  let result;

  if (type === 4) return literal(bits);
  result = operator(bits);
  bits = result[0];
  switch(type){
    case 0: // sum
      let sum = result[1].reduce((c,p) => p+c,0);
      return [bits, sum];
    case 1: // product
      let product = result[1].reduce((c,p) => c*p,1);
      return [bits, product];
    case 2: // minimum
      return [bits, Math.min(...result[1])];
    case 3: // maximum
      return [bits, Math.max(...result[1])];
    case 5: // greater than
      let gt = result[1][0] > result[1][1] ? 1 :0;
      return [bits, gt];
    case 6: // lesser than
      let lt = result[1][0] < result[1][1] ? 1 :0;
      return [bits, lt];
    case 7: // equal
      let eq = result[1][0] == result[1][1] ? 1 :0;
      return [bits, eq];
  }
}

const decoder = hex => {
  let bits = hex.split('').reduce((p,c) => {
    p += parseInt(c,16).toString(2).padStart(4,0);
    return p;
  },'').split('');
  bits = decode(bits);
  console.log(`result ${bits[1]}`);
}

//decoder("D2FE28"); ok
//decoder("38006F45291200"); ok
//decoder("EE00D40C823060"); ok
//decoder("8A004A801A8002F478"); ok
//decoder("620080001611562C8802118E34"); ok
//decoder("C0015000016115A2E0802F182340"); ok
//decoder("A0016C880162017C3686B18A3D4780"); ok

/*--- part 2 ---*/
/*
decoder("C200B40A82") // finds the sum of 1 and 2, resulting in the value 3.
decoder("04005AC33890") // finds the product of 6 and 9, resulting in the value 54.
decoder("880086C3E88112") // finds the minimum of 7, 8, and 9, resulting in the value 7.
decoder("CE00C43D881120") // finds the maximum of 7, 8, and 9, resulting in the value 9.
decoder("D8005AC2A8F0") // produces 1, because 5 is less than 15.
decoder("F600BC2D8F") // produces 0, because 5 is not greater than 15.
decoder("9C005AC2F8F0") // produces 0, because 5 is not equal to 15.
decoder("9C0141080250320F1802104A08") // produces 1, because 1 + 3 = 2 * 2.
*/

decoder(require('fs').readFileSync("decoder.input", { encoding: "utf-8" }));
console.log(`Version total: ${sumVersion}`);

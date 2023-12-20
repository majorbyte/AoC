const fs = require('fs');

const node = d => {
    return {direction: d[0], length:Number(d[1]), color:d[2]};
}

const node2 = d => {

    let dir = '';
    switch(d[2][7]){
        case '0': dir ='R';break;
        case '1': dir ='D';break;
        case '2': dir ='L';break;
        case '3': dir ='U';break;
    }
    let hex = parseInt(d[2].substr(2,5),16);
    return {direction: dir, length:hex, color:d[2]};
}


const printNode = (node)  => {
    console.log(node.join(""));
}

const map = data => {
    let path = data.split('\r\n').map(l => node(l.split(" ")));



    const w = path.filter(x => x.direction == 'L' || x.direction == 'R').reduce((p,c) => {
        if (c.direction == 'L' && p.x - c.length >= 0) {
            p.x -= c.length;
        } else if (c.direction == 'L' && p.x - c.length < 0) {
            p.w += Math.abs(p.x-c.length);
            p.x -= c.length;
            if (p.x < p.m) p.m = p.x;
        } 
        if (c.direction == 'R' && p.x + c.length > p.w ) {
            p.w += c.length + p.x - p.w;
            p.x = p.w;
        } else if (c.direction == 'R'){
            p.x += c.length;
        }

        return p;
    },{w:0, x:0,m:0});

    const h = path.filter(x => x.direction == 'U' || x.direction == 'D').reduce((p,c) => {
        if (c.direction == 'U' && p.x - c.length >= 0) {
            p.x -= c.length;
        } else if (c.direction == 'U' && p.x - c.length < 0) {
            p.w += Math.abs(p.x-c.length);
            p.x -= c.length;
            if (p.x < p.m) p.m = p.x;
        } else if (c.direction == 'D' && p.x + c.length > p.w ) {
            p.w += c.length + p.x - p.w;
            p.x = p.w;
        } else if (c.direction == 'D'){
            p.x += c.length;
        }

        return p;
    },{w:0, x:0,m:0});


    let x = 0;
    let y = 0;

    const width = w.w+1;
    const x_offset = Math.abs(w.m);
    const height = h.w+1;
    const y_offset = Math.abs(h.m);
    
    let trench = [];
    for (let n=0; n < height;n++)
        trench.push(".".repeat(width).split(""));
    
    console.dir(w);
    console.dir(h);

    path.map(node => {
        //console.dir(node);
        switch (node.direction){
            case 'R':
                for(let n = 0; n <= node.length;n++){
                    trench[y+y_offset][x+x_offset+n] = "#";
                }
                x += node.length;

                break;
            case 'L':
                for(let n = 0; n <= node.length;n++){
                    trench[y+y_offset][x+x_offset-node.length+n] = "#";
                }
                x-=node.length;

                break;

            case 'U':
                for(let n = 0; n <= node.length;n++){
                    trench[y+y_offset-node.length+n][x+x_offset] = "#";
                }
                y-=node.length;
                break;
            case 'D':
                
                for(let n = 0; n <= node.length;n++){
                    trench[y+y_offset+n][x+x_offset] = "#";
                }
                y+=node.length;
                break;
        }

    });
    

        const to_visit = [[13,99]];


        while (to_visit.length > 0){

            let coords = to_visit.pop();
            let y = coords[0], x = coords[1];
            if (trench[y][x+1] == '.') to_visit.push([y,x+1]);
            if (trench[y][x-1] == '.') to_visit.push([y,x-1]);
            if (trench[y+1][x] == '.') to_visit.push([y+1,x]);
            if (trench[y-1][x] == '.') to_visit.push([y-1,x]);
    
            trench[y][x] ='#';
        }

    
    let s = trench.map(n => n.join("")).join("");
    s = s.replaceAll(".","");
    console.log(s.length);
    //console.log(risk);
    //console.log(volumes.sort((a,b) => a < b ? 1 : -1).slice(0,3).reduce((p,c)=> p*c,1));
}


const map2 = data => {
    let path = data.split('\r\n').map(l => node2(l.split(" ")));


    let x = 0;
    let y = 0;

    let vertices = [[0,0]]

    path.map(node => {
        //console.dir(node);
        switch (node.direction){
            case 'R': x += node.length;break;
            case 'L': x -= node.length;break;
            case 'U': y -= node.length;break;
            case 'D': y += node.length;break;
        }
        vertices.push([x,y])
    });

    let left = BigInt(0)
    let right = BigInt(0)
  
    for (let i = 0; i < vertices.length - 1; i++) {
      left += BigInt(vertices[i][0]) * BigInt(vertices[i + 1][1]);
      right += BigInt(vertices[i][1]) * BigInt(vertices[i + 1][0]);
    }
  
    let shoelace = (right - left) / BigInt(2)
  
    if (shoelace < 0) shoelace = shoelace * BigInt(-1);

    const length = path.reduce((p,c) => p + c.length, 0)
  
    console.log( shoelace + BigInt(length) / BigInt(2) + BigInt(1))
    
}

fs.readFile('./input.txt', 'utf-8', (_,data) => map(data));
fs.readFile('./input.txt', 'utf-8', (_,data) => map2(data));

// 106919930664458 too low
// 106920098354636

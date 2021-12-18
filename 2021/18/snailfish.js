class TreeNode {
	constructor(depth, parent, left, right, value) {
		this.depth = depth || 0;
		this.value = value || null;
		this.left = left || null;
		this.right = right || null;
		this.parent = parent || null;
	}

	magnitude = _ => this.value !== null ? this.value : this.left.magnitude() * 3 + this.right.magnitude() * 2;

	toArray = _ => this.value !== null ? this.value : [this.left.toArray(), this.right.toArray()];

	nextToSplit = _ => this.value !== null ? this.value >= 10 ? this : null : this.left.nextToSplit() || this.right.nextToSplit();

	rightmostValue = _ => this.value !== null ? this : this.right.rightmostValue();

	leftmostValue = _ => this.value !== null ? this : this.left.leftmostValue();

	leftAdjacent = _ =>  this.parent ? (this.parent.left !== this ? this.parent.left.rightmostValue() : this.parent.leftAdjacent() ) : null;
  
	rightAdjacent = _ => this.parent ? (this.parent.right !== this ? this.parent.right.leftmostValue() : this.parent.rightAdjacent() ) : null;

	nextToExplode() {
		if(this.value !== null) return null;
		if(this.depth === 4) return this;

		const l = this.left.nextToExplode();
		if(l && l.value === null) return l;
		
    const r = this.right.nextToExplode();
		if(r && r.value === null) return r;
		
    return null;
	}

	explode() {
		const leftAdj = this.leftAdjacent();
		const rightAdj = this.rightAdjacent();
		if(leftAdj !== null) leftAdj.value += this.left.value;
		if(rightAdj !== null) rightAdj.value += this.right.value;
		this.left = this.right = null;
		this.value = 0;
	}

	split() {
		this.left = new TreeNode(this.depth +1, this, null, null, Math.floor(this.value/2));
		this.right = new TreeNode(this.depth +1, this, null, null, Math.ceil(this.value/2));
		this.value = null;
	}

  reduce() {
    let [explode, split] = [this.nextToExplode(), this.nextToSplit()];
    while (explode || split){
      if(explode) explode.explode();
			else if(split) split.split();
      [explode, split] = [this.nextToExplode(), this.nextToSplit()];
    }
		return this;
	}
}

const createTree = (depth, from, parent) => {
	const newTree = new TreeNode(depth, parent);
	if(isNaN(from)){
		newTree.left = createTree(depth +1 , from[0], newTree);
		newTree.right = createTree(depth +1, from[1], newTree);
  } else {
		newTree.value = from;
	}
	return newTree;
};

const reduce = (curr, newNode) => {
  const mainTree = new TreeNode(0, null,curr);
  curr.parent = mainTree;
  mainTree.right = createTree(0, newNode, mainTree);
  return createTree(0, mainTree.toArray()).reduce();
}

const part1 = data => data.reduce((p,c) => reduce(p, c) , createTree(0,data.shift()));

const part2 = data => data.reduce((p,c) => {
  const left = createTree(0,c);
  const result = data.reduce((x, y) => {
    if (c === y) return x;
    const result = reduce(left, y);
    return x < result.magnitude() ?  result.magnitude() : x;
  }, p)
  return p < result ?  result : p;
}, 0);

let result = part1(JSON.parse(`[${require('fs').readFileSync("snailfish.input", { encoding: "utf-8" })}]`.split('\r\n').join(',') ));
console.log(result.magnitude());

result = part2(JSON.parse(`[${require('fs').readFileSync("snailfish.input", { encoding: "utf-8" })}]`.split('\r\n').join(',') ));
console.log(result);

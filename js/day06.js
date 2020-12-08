console.log(
    require("fs")
    .readFileSync("data/day6.txt", "utf-8")
    .split(/(\r?\n){2}/)
    .map(x=>x.trim().split("\n").map(x=>x.split('')))
    .map(x=>[x.reduce((a,x)=>new Set([...a,...x]),new Set()),x.map(x=>new Set(x))])
    .map(([a,b])=>[a.size,b.reduce((a,b)=>new Set([...a].filter(x =>b.has(x))),b[0]).size])
    .reduce(([a,b],[c,d])=>[a+c,b+d],[0,0]));

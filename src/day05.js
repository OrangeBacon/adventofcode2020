console.log(
    require("fs")
    .readFileSync("data/day5.txt", "utf-8")
    .split("\n")
    .filter(x=>x.length>0)
    .map(x=>parseInt(x.trim().replace(/[FBLR]/g,x=>({F:0,B:1,L:0,R:1})[x]),2))
    .sort((a,b)=>a-b)
    .reduce((a,v,i,t)=>[t,t[0],t.reverse()[0], t[0]])
    .reduce((a,v,i,t)=>[t[2],[...Array(t[2]-t[1]).keys()].map(x=>x+t[1]),t[0]])
    .reduce((a,v,i,t)=>[t[0],t[1].filter(x=>!t[2].includes(x))[0]]));
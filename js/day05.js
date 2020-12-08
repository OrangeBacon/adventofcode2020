console.log(
    require("fs")
    .readFileSync("data/BigBigDay5.txt", "utf-8")
    .trim()
    .split("\n")
    .map(x=>parseInt(x.trim().replace(/[FBLR]/g,x=>+"BR".includes(x)),2))
    .sort((a,b)=>a-b)
    .reduce(([p,m],v)=>[v,v-p-2?m:v-1],[]));
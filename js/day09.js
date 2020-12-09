console.log((
    n=require("fs").readFileSync("data/day9.txt", "utf-8").trim().split("\n")
    .map(x=>+x),z=(a,b)=>a+b,l=n.length,a=Array,m=Math,p=a(l-25).fill(0).map(
    (_,i,x)=>(w=n.slice(i,i+25),t=n[i+25],[t,w.flatMap(a=>w.map(b=>a+b,0)).find
    (x=>x==t)])).find(([_,x])=>!x)[0],[p,a(l-1).fill(0).map((_,s)=>a(l-s-1).
    fill(0).map((_,i)=>(w=n.slice(i,i+s+2),(w.reduce(z)==p)*(m.max(...w)+m.min
    (...w)))).reduce(z)).reduce(z)]));
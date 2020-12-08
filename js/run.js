console.log((
    m=process.hrtime.bigint,t=m(),f=i=>`js/day${i>8?i+1:"0"+(i+1)}.js`,
    z=require("fs"),r=(x=Array(25).fill(0).map((_,i)=>[i+1,z.existsSync(f(i))])
    .filter(([i,x])=>x).map(([x])=>x),p=process.argv,l=x.length-1,g=p.length,
    s="Day number too ",g==2?[x[l]]:g!=3?"Too many args":p[2]=="all"?x:(n=+p[2]
    ,isNaN(n)?s+"non-numeric":n>l+1?s+"large":n<1?s+"small":[n])),typeof r==
    "string"?r:r.forEach(e=>eval(z.readFileSync(f(e-1),"utf-8"))),
    (Number(m()-t)*1e-9).toFixed(7)+"s"))

console.log(
    require("fs")
    .readFileSync("data/day8.txt", "utf-8")
    .trim().split("\n")
    .map(x=>x.trim().split(" "))
    .map(([o,v],i)=>[o=='acc'?0:o=='jmp'?1:2,+v])
    .reduce((a,b,c,d)=>[
        c=>c.map(([o,a],i)=>
        `case ${i}:${o==0?`a+=${a}`:o==1?`i+=${a-1}`:""};break;`)
        .reduce(([a,b],c)=>[a+c,b+1],["",0])
        .reduce((s,c)=>eval(`x=Array(${c}).fill(0);a=0;i=0;for(;;){if(i>${c-1}
            ||x[i])break;x[i]++;switch(i++){${s}}}[a,x[i]]`)),
    d,d]).map((x,i,[f])=>i==0?0:i==1?f(x)[0]:x.map((_,i,a,j=JSON)=>
        f(x=j.parse(j.stringify(a)),y=x[i][0],x[i][0]=y==0?y:y==1?2:1))
        .filter(([_,b])=>!b)[0][0]).slice(1));


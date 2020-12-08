console.log(
    require('fs')
    .readFileSync('data/day1.txt', 'utf8')
    .split('\n')
    .filter(x=>x.length>0)
    .map(x=>+x)
    .map((n,i,a)=>a
        .map((s,j)=>[
            n+s==2020&&j>i?n*s:0,
            a.map((t,k)=>n+s+t==2020&&k>j&&j>i?n*s*t:0)
                .reduce((p,v)=>p+v)])
        .reduce((p,v)=>[p[0]+v[0],p[1]+v[1]],[0,0]))
    .reduce((p,v)=>[p[0]+v[0],p[1]+v[1]],[0,0]));

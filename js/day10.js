console.log(
    require("fs").readFileSync("data/day10.txt", "utf-8")
    .trim().split("\n").map(x=>+x).sort((a,b)=>a-b)
    .reduce((a,b,c,d)=>[d,d]).map((a,i)=>i?(u=Math.max(...a),f=[0,...a,u+3],
    q=f.map((x,i)=>(o=[],s=1,f.slice(i+1).map((z,j)=>s&&z-x<4?o.push(j+i+1):
    s=0),o)).filter(x=>x.length),m=[],((f,a)=>f(f,a))((f,i)=>m[i]=q[i].reduce
    ((c,n)=>c+(n==q.length?1:m[n]?m[n]:f(f,n)),0),0)):(o=0,f=0,t=1,a.map(a=>
    (d=a-o,o=a,d==1?f++:d==3?t++:0)),f*t)));
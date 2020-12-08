console.log(
    [require("fs")
    .readFileSync("data/day7.txt","utf-8")
    .trim().split("\n")
    .map(x=>x.trim().split("contain"))
    .map(([n,l],a,b,z=/ bags?\.*/)=>[
        n.trim().replace(z,""),
        l.split(",").map(x=>x.trim().replace(z,"").split(/(\d+) (.*)/))
            .map(([,b,c])=>[b=="no"?null:+b,c]).filter(([a])=>a)])
    .reduce((a,[n,v])=>(a[n]=v)&&a,{})]
    .reduce((a,b,c,[d])=>[d,d],0)
    .map((x,i,_,z="shiny gold")=>i?((f,a)=>f(f,a))((f,b)=>b
        .reduce((a,[n,v])=>a+n*f(f,x[v]),1), x[z])-1:(
        Object.values(x).reduce((a,n)=>((f,a)=>f(f,a))((f,b)=>
            b.reduce((a,[,v])=>a||v==z||f(f,x[v]),0),n)?a+1:a,0))));

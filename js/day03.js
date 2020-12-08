console.log(
    [[[3,1]],[[1,1],[3,1],[5,1],[7,1],[1,2]]]
    .map(c=>c
        .reduce((p,x)=>p*require("fs")
            .readFileSync('data/day3.txt', 'utf8')
            .split("\r\n")
            .filter(x=>x.length>0)
            .map(x=>x.split(''))
            .reduce((p,c,i)=>[(i%x[1]==0)?(c[p[1]]=='#')+p[0]:p[0],(i%x[1]==0)?(p[1]+x[0])%c.length:p[1]],[0,0])[0],1)));
            
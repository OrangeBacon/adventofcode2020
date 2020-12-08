console.log(
    require("fs")
    .readFileSync('data/day2.txt', 'utf8')
    .split("\n")
    .filter(x=>x.length>0)
    .map(x=>/(\d+)-(\d+) (.): (.+)/.exec(x))
    .map(x=>[+x[1], +x[2], x[3], x[4].split('')])
    .map((x, i)=>(i = x[3].reduce((a,v)=>(v == x[2] ? a + 1 : a), 0),
        [i>=x[0]&&i<=x[1], (x[3][x[0]-1] == x[2]) ^ (x[3][x[1]-1] == x[2])]))
    .reduce((a,v)=>[a[0]+v[0],a[1]+v[1]],[0,0]));

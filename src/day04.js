console.log(
    require('fs')
    .readFileSync('data/day4.txt', 'utf-8')
    .split("\n\n")
    .map(x=>x.split(/[ \n]/).map(x=>x.split(':')))
    .filter(x=>x.length==8||(x.length==7&&!x.find(y=>y[0]=="cid")))
    .map(x=>[1, x.map(([k,v])=>
        k=="byr"?/19[2-9][0-9]|200[0-2]/.test(v):
        k=="iyr"?/201[0-9]|2020/.test(v):
        k=="eyr"?/202[0-9]|2030/.test(v):
        k=="hgt"?/^(?:1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)$/.test(v):
        k=="hcl"?/#[0-9a-f]{6}/.test(v):
        k=="ecl"?/amb|blu|brn|gry|grn|hzl|oth/.test(v):
        k=="pid"?/^[0-9]{9}$/.test(v):
        true).reduce((a,b)=>a&b,1)])
    .reduce((p,v)=>[p[0]+v[0],p[1]+v[1]], [0,0]));
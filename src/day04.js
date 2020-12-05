console.log(
    require('fs')
    .readFileSync('data/day4.txt', 'utf-8')
    .split("\n\n")
    .map(x=>x.split(/[ \n]/).map(x=>x.split(':')))
    .filter(x=>x.length==8-!x.some(y=>y[0]=="cid"))
    .reduce(([a,b],x)=>[a+1, b+x.map(([k,v])=>
        ({byr:/19[2-9][0-9]|200[0-2]/,
        iyr:/201[0-9]|2020/,
        eyr:/202[0-9]|2030/,
        hgt:/^(?:1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)$/,
        hcl:/#[0-9a-f]{6}/,
        ecl:/amb|blu|brn|gry|grn|hzl|oth/,
        pid:/^[0-9]{9}$/,
        cid:/.*/})[k]?.test(v)?1:0).reduce((a,b)=>a&b)],[0,0]));
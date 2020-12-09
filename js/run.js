console.log((
    M=process.hrtime.bigint,T=M(),F=I=>`js/day${I>8?I+1:"0"+(I+1)}.js`,
    Z=require("fs"),R=(X=Array(25).fill(0).map((_,I)=>[I+1,Z.existsSync(F(I))])
    .filter(([I,X])=>X).map(([X])=>X),P=process.argv,L=X.length-1,G=P.length,
    S="Day number too ",G==2?[X[L]]:G!=3?"Too many args":P[2]=="all"?X:(N=+P[2]
    ,isNaN(N)?S+"non-numeric":N>L+1?S+"large":N<1?S+"small":[N])),typeof R==
    "string"?R:R.forEach(E=>eval(Z.readFileSync(F(E-1),"utf-8"))),
    (Number(M()-T)*1e-9).toFixed(7)+"s"))

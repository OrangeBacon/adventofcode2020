use anyhow::Result;
use libaoc::{aoc, time, AocResult, Instruction, VM};
use std::time::Instant;

#[aoc("1915", "944")]
pub fn solve(input: String) -> Result<AocResult> {
    let parse = Instant::now();
    let vm = VM::file_parse(&input);
    let parse = parse.elapsed().as_secs_f64();

    let (part1, t1) = time(|| match vm.clone().run() {
        Ok(a) => a,
        Err(a) => a,
    });

    let t2 = Instant::now();
    let mut part2 = 0;
    let mut fixed_stream: Vec<_> = vm.into_iter().collect();
    for (i, inst) in vm.into_iter().enumerate() {
        match inst {
            Instruction::Acc(_) => (),
            Instruction::Jmp(arg) => fixed_stream[i] = Instruction::Nop(arg),
            Instruction::Nop(arg) => fixed_stream[i] = Instruction::Jmp(arg),
        }

        let vm = VM::new(fixed_stream.clone());

        if let Ok(acc) = vm.run() {
            part2 = acc;
            break;
        }

        fixed_stream[i] = inst;
    }
    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}

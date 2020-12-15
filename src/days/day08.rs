use anyhow::Result;
use libaoc::{aoc, AocResult, Instruction, Timer, VM};

#[aoc("1915", "944")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let vm = VM::file_parse(&input);
    timer.lap("Parse");

    let part1 = match vm.clone().run() {
        Ok(a) => a,
        Err(a) => a,
    };
    timer.lap("Part 1");

    let mut part2 = 0;
    let mut fixed_stream: Vec<_> = vm.into_iter().collect();
    for (i, inst) in vm.into_iter().enumerate() {
        match inst {
            Instruction::Acc(_) => (),
            Instruction::Jmp(arg) => fixed_stream[i] = Instruction::Nop(arg),
            Instruction::Nop(arg) => fixed_stream[i] = Instruction::Jmp(arg),
        }

        let vm = VM::new(&fixed_stream);

        if let Ok(acc) = vm.run() {
            part2 = acc;
            break;
        }

        fixed_stream[i] = inst;
    }
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}

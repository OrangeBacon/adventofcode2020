use crate::{time, AocResult};
use anyhow::Result;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn run_instruction(inst: &[Instruction]) -> Result<i32, i32> {
    let mut acc = 0;
    let mut ip: i32 = 0;
    let mut has_run = vec![false; inst.len()];

    loop {
        if ip < 0 {
            break Err(acc);
        }
        if ip as usize >= inst.len() {
            break Ok(acc);
        }
        if has_run[ip as usize] {
            break Err(acc);
        }
        has_run[ip as usize] = true;
        match inst[ip as usize] {
            Instruction::Acc(i) => acc += i,
            Instruction::Jmp(i) => ip += i - 1,
            Instruction::Nop(_) => (),
        }
        ip += 1;
    }
}

pub fn day08(input: String) -> Result<AocResult> {
    let parse = Instant::now();
    let instructions = input.trim().lines().map(|x| x.split_once(" ").unwrap());

    let instructions: Vec<_> = instructions
        .map(|(name, arg)| {
            let num = arg.parse::<i32>().unwrap();
            match name {
                "acc" => Instruction::Acc(num),
                "jmp" => Instruction::Jmp(num),
                "nop" => Instruction::Nop(num),
                _ => panic!(),
            }
        })
        .collect();
    let parse = parse.elapsed().as_secs_f64();

    let (part1, t1) = time(|| match run_instruction(&instructions) {
        Ok(a) => a,
        Err(a) => a,
    });

    let t2 = Instant::now();
    let mut part2 = 0;
    let mut fixed_stream = instructions.clone();
    for (i, inst) in instructions.iter().enumerate() {
        match inst {
            Instruction::Acc(_) => (),
            Instruction::Jmp(arg) => fixed_stream[i] = Instruction::Nop(*arg),
            Instruction::Nop(arg) => fixed_stream[i] = Instruction::Jmp(*arg),
        }

        if let Ok(acc) = run_instruction(&fixed_stream) {
            part2 = acc;
            break;
        }

        fixed_stream[i] = *inst;
    }
    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}

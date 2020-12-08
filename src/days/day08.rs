use crate::{time, AocResult, Instruction, VM};
use anyhow::Result;
use std::time::Instant;

pub fn day08(input: String) -> Result<AocResult> {
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

#[cfg(test)]
mod test {
    use crate::days::*;
    use anyhow::Result;

    #[test]
    fn day07a() -> Result<()> {
        let res = day08::day08(DEFAULT_DATA[7].to_string())?;
        assert_eq!(res.part1, "1915");
        Ok(())
    }

    #[test]
    fn day07b() -> Result<()> {
        let res = day08::day08(DEFAULT_DATA[7].to_string())?;
        assert_eq!(res.part2, "944");
        Ok(())
    }
}

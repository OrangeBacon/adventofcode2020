use std::iter::IntoIterator;

/// VM instructions and their arguments
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

use Instruction::*;

impl Instruction {
    /// create a new instruction from the string descriptors of it
    fn new((name, arg): (&str, &str)) -> Self {
        let num = arg.parse::<i32>().unwrap();
        match name {
            "acc" => Acc(num),
            "jmp" => Jmp(num),
            "nop" => Nop(num),
            _ => panic!(),
        }
    }

    /// run the instruction on the passed virtual machine
    fn run(&mut self, vm: &mut VM) {
        match self {
            Acc(i) => vm.acc += *i,
            Jmp(i) => vm.ip += *i - 1,
            Nop(_) => (),
        }
    }
}

/// the mapping between instruction and whether they have been run yet
/// used for infinite loop checking
#[derive(Clone, Copy)]
pub struct InstructionState {
    pub inst: Instruction,
    has_run: bool,
}

impl InstructionState {
    /// construct a state from an instruction
    fn new(inst: Instruction) -> Self {
        InstructionState {
            has_run: false,
            inst: inst,
        }
    }
}

/// vitual machine, stores state and dispatches running instructions
#[derive(Clone)]
pub struct VM {
    instructions: Vec<InstructionState>,

    acc: i32,
    ip: i32,
}

impl VM {
    /// parse an assembly file into a new virtual machine
    pub fn file_parse(inst: &str) -> Self {
        Self::from_list(inst.trim().lines().map(|x| x.split_once(" ").unwrap()))
    }

    /// turn parsed assembly stream into instructions
    pub fn from_list<'b, T>(inst: T) -> Self
    where
        T: IntoIterator<Item = (&'b str, &'b str)>,
    {
        Self::new(
            inst.into_iter()
                .map(|x| Instruction::new(x))
                .collect::<Vec<_>>(),
        )
    }

    /// construct a virtual machine from a list of instructions
    pub fn new(inst: Vec<Instruction>) -> Self {
        VM {
            instructions: inst.iter().map(|&x| InstructionState::new(x)).collect(),
            acc: 0,
            ip: 0,
        }
    }

    /// run the code in the virtual machine
    /// returns the accumulator value
    /// if returns to an instruction, then assumes infinite loop and errors
    pub fn run(mut self) -> Result<i32, i32> {
        loop {
            if self.ip < 0 {
                break Err(self.acc);
            }
            if self.ip as usize >= self.instructions.len() {
                break Ok(self.acc);
            }

            let mut inst = self.instructions[self.ip as usize];
            if inst.has_run {
                break Err(self.acc);
            }
            inst.has_run = true;
            inst.inst.run(&mut self);
            self.instructions[self.ip as usize] = inst;

            self.ip += 1;
        }
    }
}

impl<'a> IntoIterator for &'a VM {
    type Item = Instruction;
    type IntoIter = VMInstructionIter<'a>;
    fn into_iter(self) -> VMInstructionIter<'a> {
        VMInstructionIter {
            state: self,
            index: 0,
        }
    }
}

pub struct VMInstructionIter<'a> {
    state: &'a VM,
    index: usize,
}

impl<'a> Iterator for VMInstructionIter<'a> {
    type Item = Instruction;
    fn next(&mut self) -> Option<Instruction> {
        self.index += 1;
        self.state
            .instructions
            .get(self.index - 1)
            .and_then(|&f| Some(f.inst))
    }
}

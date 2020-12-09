use std::{collections::HashSet, iter::FromIterator};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Instruction {
    ACC(isize), // add argument to acc
    JMP(isize), // jump to current + offset
    NOP(isize), // don't do anything
}

impl From<&str> for Instruction {
    fn from(data: &str) -> Self {
        let v: Vec<_> = data.split_whitespace().collect();
        match v[0] {
            "acc" => Instruction::ACC(v[1].parse().unwrap()),
            "jmp" => Instruction::JMP(v[1].parse().unwrap()),
            "nop" => Instruction::NOP(v[1].parse().unwrap()),
            _ => panic!("Invalid command"),
        }
    }
}

#[derive(Debug, Clone)]
struct ProgramState {
    acc: isize,
    ip: usize,
    msg: Result<&'static str, &'static str>,
}

struct Program {
    instructions: Vec<Instruction>,
}

impl From<Vec<Instruction>> for Program {
    fn from(instructions: Vec<Instruction>) -> Self {
        Program {
            instructions
        }
    }
}

impl From<&Vec<Instruction>> for Program {
    fn from(instructions: &Vec<Instruction>) -> Self {
        Program {
            instructions: instructions.clone()
        }
    }
}

impl FromIterator<Instruction> for Program {
    fn from_iter<T: IntoIterator<Item = Instruction>>(iter: T) -> Self {
        Program {
            instructions: iter.into_iter().collect()
        }
    }
}

impl Program {
    fn run(&mut self) -> ProgramState {
        use std::convert::TryFrom;
        use std::collections::HashMap;

        let mut ip_counter: HashMap<usize, usize> = HashMap::new();
        let mut ip = 0;
        let mut acc = 0;

        while ip < self.instructions.len() {
            println!("IP: {:>5} | ACC: {:>5} | INST: {:?}", ip, acc, self.instructions[ip]);

            let entry: &mut usize = ip_counter.entry(ip).or_insert(0usize);
            if *entry > 0 {
                return ProgramState{acc, ip, msg: Err("Infinite Loop")};
            }
            *entry += 1;

            match &self.instructions[ip] {
                Instruction::ACC(v) => {
                    acc += v;
                    ip += 1;
                }
                Instruction::JMP(v) => {
                    if *v >= 0 {
                        ip += usize::try_from(*v).unwrap();
                    } else {
                        ip -= usize::try_from(v.abs()).unwrap();
                    }
                }
                Instruction::NOP(_) => {
                    ip += 1;
                }
            }
        }
        ProgramState{acc, ip, msg: Ok("Succesful termination")}
    }
}

pub fn part_01(data: &str) -> isize {
    let mut program = Program::from_iter(data.lines().map(Instruction::from));
    let state = program.run();
    if let Err(_) = state.msg {
        eprintln!("{:?}", state);
        return state.acc;
    }
    0
}

pub fn part_02(data: &str) -> isize {
    let instructions: Vec<_> = data.lines().map(Instruction::from).collect();
    let mut new_instructions = instructions.clone();
    let mut changed_instructions = HashSet::<usize>::new();

    loop {
        let maybe_instr = instructions.iter()
            .enumerate()
            .find(|(ip, i)| {
                let permutable = match **i {
                    Instruction::ACC(_) => false,
                    _ => true,
                };
                permutable && !changed_instructions.contains(ip)
            });

        if let Some(instr) = maybe_instr {
            println!("Replacing: {:?}", instr);
            match instr.1 {
                Instruction::JMP(v) => {
                    new_instructions[instr.0] = Instruction::NOP(*v);
                },
                Instruction::NOP(v) => {
                    new_instructions[instr.0] = Instruction::JMP(*v);
                },
                _ => (),
            };
            changed_instructions.insert(instr.0);

            let mut program = Program::from(&new_instructions);
            let result = program.run();
            if let Err(_) = result.msg {
                // swap back
                match instr.1 {
                    Instruction::JMP(v) => {
                        new_instructions[instr.0] = Instruction::JMP(*v);
                    },
                    Instruction::NOP(v) => {
                        new_instructions[instr.0] = Instruction::NOP(*v);
                    },
                    _ => (),
                };
            } else {
                println!("{:?}", result);
                return result.acc;
            }
        }
    }
}
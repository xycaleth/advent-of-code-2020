use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone)]
enum Op {
    NoOp(i32),
    Acc(i32),
    Jmp(i32),
}

struct OpInfo {
    op: Op,
    visited: bool,
}

struct Program {
    ops: Vec<OpInfo>,
    accumulator: i32,
    program_counter: i32,
}

impl OpInfo {
    fn new(op: Op) -> Self {
        OpInfo {
            op: op,
            visited: false,
        }
    }
}

impl Program {
    fn new() -> Self {
        Program {
            ops: Vec::new(),
            accumulator: 0,
            program_counter: 0,
        }
    }

    fn parse_instruction(&mut self, line: &str) {
        let mut tokens = line.split_ascii_whitespace();

        let op = tokens.next().unwrap();
        let num = tokens.next().unwrap().parse::<i32>().unwrap();

        match op {
            "acc" => self.ops.push(OpInfo::new(Op::Acc(num))),
            "nop" => self.ops.push(OpInfo::new(Op::NoOp(num))),
            "jmp" => self.ops.push(OpInfo::new(Op::Jmp(num))),
            _ => (),
        }
    }

    fn run_until_end(&mut self) -> bool {
        self.reset();

        while (self.program_counter as usize) < self.ops.len()
            && !self.ops[self.program_counter as usize].visited
        {
            let mut op = &mut self.ops[self.program_counter as usize];
            match op.op {
                Op::NoOp(_) => {
                    self.program_counter += 1;
                }
                Op::Acc(amount) => {
                    self.accumulator += amount;
                    self.program_counter += 1;
                }
                Op::Jmp(offset) => {
                    self.program_counter += offset;
                }
            }

            op.visited = true;
        }

        return (self.program_counter as usize) == self.ops.len();
    }

    fn reset(&mut self) {
        self.program_counter = 0;
        self.accumulator = 0;
        for i in 0..self.ops.len() {
            self.ops[i].visited = false;
        }
    }

    fn run_with_patch(&mut self, op_index: usize, new_op: Op) -> bool {
        let original_op = self.ops[op_index].op.clone();

        self.ops[op_index].op = new_op;
        let finished = self.run_until_end();
        self.ops[op_index].op = original_op;

        finished
    }

    fn tweak_until_finished(&mut self) {
        for i in 0..self.ops.len() {
            match self.ops[i].op {
                Op::NoOp(num) => {
                    if self.run_with_patch(i, Op::Jmp(num)) {
                        break;
                    }
                }
                Op::Jmp(num) => {
                    if self.run_with_patch(i, Op::NoOp(num)) {
                        break;
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("../input8.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut program = Program::new();
    for line in reader.lines().filter_map(Result::ok) {
        program.parse_instruction(&line);
    }

    program.run_until_end();
    println!("{}", program.accumulator);

    program.reset();
    program.tweak_until_finished();
    println!("{}", program.accumulator);

    Ok(())
}

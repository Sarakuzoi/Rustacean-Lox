use std::env;

use lazy_static::lazy_static;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::compile,
    debug::dissasemble_instruction,
    value::{print_value, Value},
};

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

// static DEBUG_TRACE_EXECUTION: bool = env::var("DEBUG_TRACE").unwra;
lazy_static! {
    static ref DEBUG_TRACE_EXECUTION: bool = env::var("DEBUG_TRACE")
        .unwrap_or(String::from("false"))
        .parse()
        .unwrap_or(false);
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum InterpretResult {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        // self.chunk = chun.clone();
        // self.ip = chunk.code.clone();
        let chunk = Chunk::new();

        if !compile(source, &chunk) {
            InterpretResult::INTERPRET_COMPILE_ERROR;
        }

        self.chunk = chunk;
        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            if *DEBUG_TRACE_EXECUTION {
                print!("          ");
                for slot in self.stack.iter() {
                    print!("[ ");
                    print_value(Some(slot));
                    print!(" ]");
                }
                println!();
                dissasemble_instruction(&self.chunk, self.ip);
            }
            let instruction =
                num::FromPrimitive::from_u8(self.read_byte()).expect("Unknown OpCode");
            match instruction {
                OpCode::OP_CONSTANT | OpCode::OP_CONSTANT_LONG => {
                    let constant = self.read_constant(instruction);
                    self.push(constant.expect("Constant was not read properly"));
                }
                OpCode::OP_ADD => self.binary_op("+"),
                OpCode::OP_SUBTRACT => self.binary_op("-"),
                OpCode::OP_MULTIPLY => self.binary_op("*"),
                OpCode::OP_DIVIDE => self.binary_op("/"),
                OpCode::OP_NEGATE => {
                    let aux = self.pop();
                    self.push(-aux);
                }
                OpCode::OP_RETURN => {
                    print_value(Some(&self.pop()));
                    println!();
                    return InterpretResult::INTERPRET_OK;
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let ret = *self
            .chunk
            .code
            .get(self.ip)
            .expect("Instruction Pointer out of bounds!");
        self.ip += 1;
        ret
    }

    fn read_constant(&mut self, instruction_type: OpCode) -> Option<Value> {
        let jmp = match instruction_type {
            OpCode::OP_CONSTANT => 1,
            OpCode::OP_CONSTANT_LONG => 3,
            _ => unreachable!(),
        };

        let index: usize = match instruction_type {
            OpCode::OP_CONSTANT => {
                (*self.chunk.code.get(self.ip).expect("Offset out of bounds")).into()
            }
            OpCode::OP_CONSTANT_LONG => {
                let index = *self.chunk.code.get(self.ip).expect("Offset out of bounds") as u32
                    | (*self
                        .chunk
                        .code
                        .get(self.ip + 1)
                        .expect("Offset out of bounds") as u32)
                        << 8
                    | (*self
                        .chunk
                        .code
                        .get(self.ip + 2)
                        .expect("Offset out of bounds") as u32)
                        << 16;
                index.try_into().expect("u32 does not fit into usize")
            }
            _ => unreachable!(),
        };

        self.ip += jmp;
        self.chunk.constants.values.get(index).copied()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn binary_op(&mut self, symbol: &str) {
        let b = self.pop();
        let a = self.pop();
        match symbol {
            "+" => self.push(a + b),
            "-" => self.push(a - b),
            "*" => self.push(a * b),
            "/" => self.push(a / b),
            _ => unreachable!(),
        }
    }
}

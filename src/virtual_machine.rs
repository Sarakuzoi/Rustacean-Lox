use crate::{
    chunk::{Chunk, OpCode},
    value::{print_value, Value},
};

pub struct VM {
    chunk: Chunk,
    // ip: Vec<u8>,
    ip: usize,
}

#[allow(non_camel_case_types)]
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
        }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.chunk = chunk.clone();
        // self.ip = chunk.code.clone();
        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instruction =
                num::FromPrimitive::from_u8(self.read_byte()).expect("Unknown OpCode");
            match instruction {
                OpCode::OP_RETURN => return InterpretResult::INTERPRET_OK,
                OpCode::OP_CONSTANT | OpCode::OP_CONSTANT_LONG => {
                    let constant = self.read_constant(instruction);
                    print_value(constant.as_ref());
                    println!();
                } // _ => todo!(),
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
}

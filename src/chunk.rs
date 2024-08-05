use num_derive::FromPrimitive;

use crate::value::{Value, ValueArray};

#[allow(non_camel_case_types)]
#[derive(FromPrimitive, Debug)]
pub enum OpCode {
    OP_CONSTANT,
    OP_RETURN,
}

pub struct Chunk {
    pub code: Vec<u8>, // or u8
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.write(constant);
        self.constants.values.len() - 1
    }
}
